// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test, Vm} from "forge-std/Test.sol";
import "forge-std/StdUtils.sol";
import {EvaluableConfig, Evaluable} from "rain.interpreter/lib/caller/LibEvaluable.sol";
import {IExpressionDeployerV1} from "rain.interpreter/interface/IExpressionDeployerV1.sol";
import {IInterpreterStoreV1} from "rain.interpreter/interface/IInterpreterStoreV1.sol";
import {IInterpreterV1} from "rain.interpreter/interface/IInterpreterV1.sol";
import {IFlowERC1155V3, FlowERC1155Config} from "rain.flow/interface/IFlowERC1155V3.sol";
import {SignedContextV1} from "rain.interpreter/interface/IInterpreterCallerV2.sol";
import {IERC20Metadata} from "openzeppelin-contracts/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "rain.factory/src/interface/ICloneableFactoryV2.sol";
import {Token} from "./contracts/Token.sol";
import 'forge-std/StdJson.sol';
import { Strings } from 'openzeppelin-contracts/contracts/utils/Strings.sol';
import 'forge-std/StdCheats.sol';

contract NftTest is Test {
    using stdJson for string;
    using Strings for uint256;

    address alice = makeAddr("alice");
    address joe = makeAddr("joe");
    address eve = makeAddr("eve");

    // this is hardcoded in the flow expression
    address gameMaster = 0x504093896403Aa2888e24ddE68c14e3435c2DEc5;
    Token paymentToken = Token(0x2Eb1D24aB0eC5FD0058ab5073F1EA2d8A59783E5);

    ICloneableFactoryV2 public factory = ICloneableFactoryV2(0x70dD832A82481d4e1d15A3B50Db904719e2d3341);
    address public implementation = 0x2f1a7d6dF220508b4E06e62b8D6bAdAc8e38a11C;
    IExpressionDeployerV1 public deployer = IExpressionDeployerV1(0x0a2392aB861834305dB90A8825af102C02B6929C);

    IFlowERC1155V3 public instance;

    IInterpreterStoreV1 store;
    IInterpreterV1 interpreter;

    Evaluable snapshotEvaluable;
    Evaluable mintEvaluable;
    Evaluable claimEvaluable;

    function setUp() public {
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

        address _instance = factory.clone(implementation, abi.encode(config));

        Vm.Log[] memory entries = vm.getRecordedLogs();
        (, address _interpreter, address _store, address _snapshotExp) = abi.decode(entries[4].data, (address, address, address, address));
        (, , , address _mintExp) = abi.decode(entries[7].data, (address, address, address, address));
        (, , , address _claimExp) = abi.decode(entries[10].data, (address, address, address, address));

        instance = IFlowERC1155V3(_instance);
        interpreter = IInterpreterV1(_interpreter);
        store = IInterpreterStoreV1(_store);
        
        snapshotEvaluable = Evaluable({interpreter: interpreter, store: store, expression: _snapshotExp});
        mintEvaluable = Evaluable({interpreter: interpreter, store: store, expression: _mintExp});
        claimEvaluable = Evaluable({interpreter: interpreter, store: store, expression: _claimExp});
    }

    function testSimple() public {
        // make a snapshot for some id
        uint256 contentHash = 1;
        uint256[] memory context = new uint256[](1);
        context[0] = contentHash;
        StdCheats.deal(address(paymentToken), address(alice), 1000e18);
        StdCheats.deal(address(paymentToken), address(joe), 1000e18);

        // alice mints a snapshot
        vm.startPrank(alice);
        paymentToken.approve(address(instance), 1000e18);
        instance.flow(snapshotEvaluable, context, new SignedContextV1[](0));

        // we can calculate what the nft is
        uint256 nftId = uint256(keccak256(abi.encodePacked(uint256(uint160(alice)), contentHash)));

        assertEq(instance.balanceOf(address(alice), nftId), 1);

        // joe buys a bunch
        vm.startPrank(joe);
        paymentToken.approve(address(instance), 1000e18);
        uint256[] memory joeContext = new uint256[](2);
        joeContext[0] = nftId;
        joeContext[1] = 10;
        instance.flow(mintEvaluable, joeContext, new SignedContextV1[](0));

        assertEq(instance.balanceOf(address(instance), nftId), 10);
    }
}
