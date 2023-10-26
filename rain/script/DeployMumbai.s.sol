// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "forge-std/Script.sol";
import { EvaluableConfig, Evaluable } from 'rain.interpreter/lib/caller/LibEvaluable.sol';
import "rain.factory/src/interface/ICloneableFactoryV2.sol";
import "rain.flow/interface/IFlowERC1155V3.sol";
import 'forge-std/StdJson.sol';
import { Vm } from 'forge-std/Vm.sol';
import { Strings } from 'openzeppelin-contracts/contracts/utils/Strings.sol';
import { NativeTokenFlowERC1155Caller } from '../src/NativeTokenFlowCaller.sol';


contract DeployMumbai is Script {
    ICloneableFactoryV2 public constant factory = ICloneableFactoryV2(0x70dD832A82481d4e1d15A3B50Db904719e2d3341);
    address public constant implementation = 0xC1Ef6887b8722b8B666e4C3d0EE74bDeECb098F1;
    address public constant deployer = 0x0a2392aB861834305dB90A8825af102C02B6929C;

    address public constant wmatic = 0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889;    
    using stdJson for string;
    using Strings for uint256;


    function setUp() public {}

    function getEvaluableConfig(string memory path) internal returns (EvaluableConfig memory config) {
        string memory jsonContent = vm.readFile(
            string.concat(vm.projectRoot(), path)
        );
        config.sources = jsonContent.readBytesArray('.sources');
        config.constants = jsonContent.readUintArray('.constants');
        config.deployer = IExpressionDeployerV1(deployer);
    }

    struct Addresses {
        address nativeTokenFlowCaller;
        address interpreter;
        address store;
        address instance;
        address snapshotExp;
        address mintExp;
        address claimExp;
    }

    function decodeLogs(Vm.Log[] memory entries) internal returns (Addresses memory addresses) {
        (, addresses.interpreter, addresses.store, addresses.snapshotExp) = abi.decode(entries[5].data, (address, address, address, address));
        (, , , addresses.mintExp) = abi.decode(entries[8].data, (address, address, address, address));
        (, , , addresses.claimExp) = abi.decode(entries[11].data, (address, address, address, address));
    }

    function writeAddressesToJson(Addresses memory addresses) internal {
        string memory obj = "snapshot";
        vm.serializeAddress(obj, "nativeTokenFlowCaller", addresses.nativeTokenFlowCaller);
        vm.serializeAddress(obj, "interpreter", addresses.interpreter);
        vm.serializeAddress(obj, "store", addresses.store);
        vm.serializeAddress(obj, "instance", addresses.instance);
        vm.serializeAddress(obj, "snapshot", addresses.snapshotExp);
        vm.serializeAddress(obj, "mint", addresses.mintExp);
        string memory output = vm.serializeAddress(obj, "claim", addresses.claimExp);
        string memory file = string.concat(vm.projectRoot(), "/addresses/flow-", block.number.toString(), ".json");
        vm.writeJson(output, file);
    }

    function run() public {
        vm.startBroadcast();

        NativeTokenFlowERC1155Caller nativeTokenFlowCaller = new NativeTokenFlowERC1155Caller(wmatic);

        EvaluableConfig memory snapshot = getEvaluableConfig("/src/snapshot.json");
        EvaluableConfig memory mint = getEvaluableConfig("/src/mint.json");
        EvaluableConfig memory claim = getEvaluableConfig("/src/claim.json");

        EvaluableConfig memory canTransfer;
        canTransfer.sources = new bytes[](0);
        canTransfer.constants = new uint256[](0);
        canTransfer.deployer = IExpressionDeployerV1(deployer);

        FlowERC1155Config memory config;
        config.uri = "https://gateway-hrl.holo.host/uhCkkM7ixonFmCwGm3tYnG7N0xo6M_rfRgNbujEDtTUpnW097zIBZ/{id}";
        config.evaluableConfig = canTransfer;
        config.flowConfig = new EvaluableConfig[](3);
        config.flowConfig[0] = snapshot;
        config.flowConfig[1] = mint;
        config.flowConfig[2] = claim;

        vm.recordLogs();

        address instance = factory.clone(implementation, abi.encode(config));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        Addresses memory addresses = decodeLogs(entries);

        addresses.nativeTokenFlowCaller = address(nativeTokenFlowCaller);
        addresses.instance = instance;

        writeAddressesToJson(addresses);    }
}


// source .env && forge script script/DeployMumbai.s.sol:DeployMumbai --fork-url $RPC_URL --private-key $PRIVATE_KEY --broadcast