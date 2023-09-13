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
import { IERC1155 } from 'openzeppelin-contracts/contracts/token/ERC1155/IERC1155.sol';
import { SignContext } from './contracts/SignContext.sol';
import { NativeTokenFlowERC1155Caller } from '../src/NativeTokenFlowCaller.sol';

contract NftTest is Test, SignContext {
    using stdJson for string;
    using Strings for uint256;

    address alice = makeAddr("alice");
    address joe = makeAddr("joe");
    address eve = makeAddr("eve");

    // this is hardcoded in the flow expression
    address gameMaster = 0x504093896403Aa2888e24ddE68c14e3435c2DEc5;
    Token paymentToken = Token(0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889);

    ICloneableFactoryV2 public factory = ICloneableFactoryV2(0x70dD832A82481d4e1d15A3B50Db904719e2d3341);
    address public implementation = 0x2f1a7d6dF220508b4E06e62b8D6bAdAc8e38a11C;
    IExpressionDeployerV1 public deployer = IExpressionDeployerV1(0x0a2392aB861834305dB90A8825af102C02B6929C);

    NativeTokenFlowERC1155Caller public nativeTokenFlowCaller;

    IFlowERC1155V3 public instance;
    IERC1155 public instanceAs1155;

    IInterpreterStoreV1 store;
    IInterpreterV1 interpreter;

    Evaluable snapshotEvaluable;
    Evaluable mintEvaluable;
    Evaluable claimEvaluable;

    function setUp() public {

        nativeTokenFlowCaller = new NativeTokenFlowERC1155Caller(address(paymentToken));

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
        instanceAs1155 = IERC1155(_instance);
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

        assertEq(instanceAs1155.balanceOf(address(alice), nftId), 1);

        // joe buys a bunch
        deal(joe, 1000 ether);
        vm.startPrank(joe);
        // paymentToken.approve(address(instance), 1000e18);
        uint256[] memory joeContext = new uint256[](3);
        joeContext[0] = nftId;
        joeContext[1] = 10;
        joeContext[2] = uint256(uint160(address(joe)));

        nativeTokenFlowCaller.flow{value: 10e15 wei}(instance, mintEvaluable, joeContext, new SignedContextV1[](0));
        // instance.flow(mintEvaluable, joeContext, new SignedContextV1[](0));

        assertEq(instanceAs1155.balanceOf(joe, nftId), 10);
        assertEq(paymentToken.balanceOf(address(instance)), 1e16);
        vm.stopPrank();

        // now create a coupon
        // the coupon will be a signed message with the following fields:
        // [0] the address of the claimant
        // [1] percentage of the pool to claim, as an 18 decimal number
        // [2] the token address
        // [3] the address of this contract

        uint256[] memory couponContext = new uint256[](4);
        couponContext[0] = uint256(uint160(joe));
        couponContext[1] = 5e17; // 50%
        couponContext[2] = uint256(uint160(address(paymentToken))); 
        couponContext[3] = uint256(uint160(address(instance)));

        uint256 stewardKey = vm.envUint("STEWARD_KEY");
        SignedContextV1[] memory signedContext = new SignedContextV1[](1);
        signedContext[0] = signContext(stewardKey, couponContext);

        // now joe can claim
        vm.startPrank(joe);
        instance.flow(claimEvaluable, new uint256[](0), signedContext);
    }
}
