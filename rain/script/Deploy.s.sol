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

        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/nft-flow.json");
        string memory expression = vm.readFile(path);

        EvaluableConfig memory flow;
        flow.sources = expression.readBytesArray('.sources');
        flow.constants = expression.readUintArray('.constants');
        flow.deployer = IExpressionDeployerV1(deployer);

        EvaluableConfig memory canTransfer;
        canTransfer.sources = new bytes[](0);
        canTransfer.constants = new uint256[](0);
        canTransfer.deployer = IExpressionDeployerV1(deployer);

        FlowERC1155Config memory config;
        config.uri = "";
        config.evaluableConfig = canTransfer;
        config.flowConfig = new EvaluableConfig[](1);
        config.flowConfig[0] = flow;

        vm.broadcast();
        vm.recordLogs();

        factory.clone(implementation, abi.encode(config));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        (, address interpreter, address store, address deployedExpression) = abi.decode(entries[4].data, (address, address, address, address));

        string memory obj = "log";
        vm.serializeAddress(obj, "interpreter", interpreter);
        vm.serializeAddress(obj, "store", store);
        string memory output = vm.serializeAddress(obj, "expression", deployedExpression);
        uint256 blockNumber = block.number;
        string memory file = string.concat(root, "/addresses/flow-", blockNumber.toString(), ".json");
        vm.writeJson(output, file);
    }
}


// forge script script/Deploy.s.sol:Deploy --fork-url $RPC_URL --private-key $PRIVATE_KEY --broadcast