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
    Token paymentToken = Token(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);

    ICloneableFactoryV2 public factory = ICloneableFactoryV2(0x662706Ee4196959dFBbE2a327100B96FBc343505);
    address public implementation = 0xAFdb6b495F10Ec70213412e97f8EBc1bcEd6152d;
    IExpressionDeployerV1 public deployer = IExpressionDeployerV1(0x6e8640784E7A4f576d8aA9Ba463d1741180d702d);

    NativeTokenFlowERC1155Caller public nativeTokenFlowCaller = new NativeTokenFlowERC1155Caller(0xcD043e255A805aB39D4632Aa61FEA9b6e142c6A3);

    IFlowERC1155V3 public instance = IFlowERC1155V3(0x95CDE7E9B0840874301E75de92e12b63Fd117Ff4);
    IERC1155 public instanceAs1155 = IERC1155(0x95CDE7E9B0840874301E75de92e12b63Fd117Ff4);

    IInterpreterStoreV1 store = IInterpreterStoreV1(0x0d6c4DA68fc7dD6D22920F6E655011C8f2b5B4A4);
    IInterpreterV1 interpreter = IInterpreterV1(0x334261036dF6c3dEB62a33C92D8695d9A5bAf598);

    Evaluable snapshotEvaluable;
    Evaluable mintEvaluable;
    Evaluable claimEvaluable;

    function setUp() public {
        
        snapshotEvaluable = Evaluable({interpreter: interpreter, store: store, expression: 0xCe6B3507e111D58117C20c8679840adD81752275});
        mintEvaluable = Evaluable({interpreter: interpreter, store: store, expression: 0x2aCd33a558e6E3734717742CC722E5cE5870AbA8});
        claimEvaluable = Evaluable({interpreter: interpreter, store: store, expression: 0x58218c5F81f1adEe69628De8657356348E1126A9});
    }

    function testFork() public {
        // make a snapshot for some id
        uint256 contentHash = 1;
        uint256[] memory context = new uint256[](1);
        context[0] = contentHash;
        StdCheats.deal(address(paymentToken), address(alice), 1000e18);
        StdCheats.deal(address(paymentToken), address(joe), 1000e18);

        // alice mints a snapshot
        vm.startPrank(alice);
        vm.warp(1696356779);

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

        address claimer = makeAddr("claimer");

        uint256[] memory couponContext = new uint256[](4);
        couponContext[0] = uint256(uint160(claimer));
        couponContext[1] = 5e17; // 50%
        couponContext[2] = uint256(uint160(address(paymentToken))); 
        couponContext[3] = uint256(uint160(address(instance)));

        uint256 stewardKey = vm.envUint("STEWARD_KEY");
        SignedContextV1[] memory signedContext = new SignedContextV1[](1);
        signedContext[0] = signContext(stewardKey, couponContext);

        vm.warp(1696345200 + 2 days);

        // now claimer can claim
        vm.startPrank(claimer);
        instance.flow(claimEvaluable, new uint256[](0), signedContext);
        assertEq(paymentToken.balanceOf(address(claimer)), 1e16 * 5e17 / 1e18);
    }
}
