// SPDX-License-Identifier: CAL
pragma solidity ^0.8.18;

import 'rain.flow/IFlowERC1155V3';
import {WETH} from 'contracts/token/WETH.sol';

contract NativeTokenFlowERC1155Caller {
    error ERC721TransferNotSupported();
    error ERC1155TransferNotSupported();
    error ERC20TransferNotSupported();
    error WrongValueSent();
    error CantBeTokenRecipient();

    WETH public WMATIC;

    function constructor(WETH _WMATIC) public {
        WMATIC = _WMATIC;
    }

    function flow(
        IFlowERC1155V3 contract, 
        Evaluable calldata evaluable,
        uint256[] calldata callerContext,
        SignedContextV1[] calldata signedContexts
    ) external payable public {

        FlowERC1155IOV1 memory flowPreview = contract.previewFlow(evaluable, callerContext, signedContexts);
        uint256 wrappedMaticAmount = 0

        // checks
        if (flowPreview.flow.erc721.length > 0) {
            revert ERC721TransferNotSupported();
        }
        if (flowPreview.flow.erc1155.length > 0) {
            revert ERC1155TransferNotSupported();
        }
        if (flowPreview.flow.erc20.length > 0) {
            for (uint256 i = 0; i < flowPreview.flow.erc20.length; i++) {
                if (flowPreview.flow.erc20[i].token == address(WMATIC) && flowPreview.flow.erc20[i].to == address(this) ) {
                    wrappedMaticAmount += flowPreview.flow.erc20[i].amount;
                } else {
                    revert ERC20TransferNotSupported();
                }
            }
        }
        if (msg.value != wrappedMaticAmount) {
            revert WrongValueSent();
        }
        if (flowPreview.mints.length > 0) {
            for (uint256 i = 0; i < flowPreview.mints.length; i++) {
                if (flowPreview.mints[i].account == address(this)) {
                    revert CantBeTokenRecipient();
                }
            }
        }
        // wrap the matic
        if (wrappedMaticAmount > 0) {
            WMATIC.deposit(wrappedMaticAmount);
        }
        // approve the flow contract
        WMATIC.approve(address(contract), wrappedMaticAmount);
        
        FlowERC1155IOV1 actualFlow = contract.flow(evaluable, callerContext, signedContexts);
    }
}