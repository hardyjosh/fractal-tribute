// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "forge-std/Script.sol";
import { EvaluableConfig, Evaluable } from 'rain.interpreter/lib/caller/LibEvaluable.sol';
import "rain.factory/src/interface/ICloneableFactoryV2.sol";
import "rain.flow/interface/IFlowERC1155V3.sol";
import 'forge-std/StdJson.sol';
import { Vm } from 'forge-std/Vm.sol';
import { Strings } from 'openzeppelin-contracts/contracts/utils/Strings.sol';

contract Deploy is Script {

    using stdJson for string;
    using Strings for uint256;

    ICloneableFactoryV2 public factory = ICloneableFactoryV2(0x70dD832A82481d4e1d15A3B50Db904719e2d3341);
    address public implementation = 0x2f1a7d6dF220508b4E06e62b8D6bAdAc8e38a11C;
    address public deployer = 0x0a2392aB861834305dB90A8825af102C02B6929C;

    function setUp() public {}

    function run() public {

        EvaluableConfig memory snapshot;
        string memory snapshotJson = vm.readFile(
            string.concat(vm.projectRoot(), "/src/snapshot.json")
        );
        snapshot.sources = snapshotJson.readBytesArray('.sources');
        snapshot.constants = snapshotJson.readUintArray('.constants');
        snapshot.deployer = IExpressionDeployerV1(deployer);

        EvaluableConfig memory mint;
        string memory mintJson = vm.readFile(
            string.concat(vm.projectRoot(), "/src/mint.json")
        );
        mint.sources = mintJson.readBytesArray('.sources');
        mint.constants = mintJson.readUintArray('.constants');
        mint.deployer = IExpressionDeployerV1(deployer);

        EvaluableConfig memory claim;
        string memory claimJson = vm.readFile(
            string.concat(vm.projectRoot(), "/src/claim.json")
        );
        claim.sources = claimJson.readBytesArray('.sources');
        claim.constants = claimJson.readUintArray('.constants');
        claim.deployer = IExpressionDeployerV1(deployer);

        EvaluableConfig memory canTransfer;
        canTransfer.sources = new bytes[](0);
        canTransfer.constants = new uint256[](0);
        canTransfer.deployer = IExpressionDeployerV1(deployer);

        FlowERC1155Config memory config;
        config.uri = "";
        config.evaluableConfig = canTransfer;
        config.flowConfig = new EvaluableConfig[](3);
        config.flowConfig[0] = snapshot;
        config.flowConfig[1] = mint;
        config.flowConfig[2] = claim;

        vm.broadcast();
        vm.recordLogs();

        address instance = factory.clone(implementation, abi.encode(config));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        (, address interpreter, address store, address snapshotExp) = abi.decode(entries[4].data, (address, address, address, address));
        (, , , address mintExp) = abi.decode(entries[7].data, (address, address, address, address));
        (, , , address claimExp) = abi.decode(entries[10].data, (address, address, address, address));

        string memory obj = "snpashot";
        vm.serializeAddress(obj, "interpreter", interpreter);
        vm.serializeAddress(obj, "store", store);
        vm.serializeAddress(obj, "instance", instance);
        vm.serializeAddress(obj, "snapshot", snapshotExp);
        vm.serializeAddress(obj, "mint", mintExp);
        string memory output = vm.serializeAddress(obj, "claim", claimExp);
        uint256 blockNumber = block.number;
        string memory file = string.concat(vm.projectRoot(), "/addresses/flow-", blockNumber.toString(), ".json");
        vm.writeJson(output, file);
    }
}


// source .env && forge script script/Deploy.s.sol:Deploy --fork-url $RPC_URL --private-key $PRIVATE_KEY --broadcast