// SPDX-License-Identifier: CAL
pragma solidity ^0.8.18;

import 'rain.flow/interface/IFlowERC1155V3.sol';
import {IWETH} from './IWETH.sol';

contract NativeTokenFlowERC1155Caller {
    error ERC721TransferNotSupported();
    error ERC1155TransferNotSupported();
    error ERC20TransferNotSupported();
    error WrongValueSent();
    error CantBeTokenRecipient();

    IWETH public WMATIC;

    constructor(address _wmatic) {
        WMATIC = IWETH(_wmatic);
    }

    function flow(
        IFlowERC1155V3 instance, 
        Evaluable calldata evaluable,
        uint256[] calldata callerContext,
        SignedContextV1[] calldata signedContexts
    ) external payable {

        FlowERC1155IOV1 memory flowPreview = instance.previewFlow(evaluable, callerContext, signedContexts);
        uint256 wrappedMaticAmount = 0;

        // checks
        if (flowPreview.flow.erc721.length > 0) {
            revert ERC721TransferNotSupported();
        }
        if (flowPreview.flow.erc1155.length > 0) {
            revert ERC1155TransferNotSupported();
        }
        if (flowPreview.flow.erc20.length > 0) {
            for (uint256 i = 0; i < flowPreview.flow.erc20.length; i++) {
                if (flowPreview.flow.erc20[i].to == address(this)) {
                    revert CantBeTokenRecipient();
                }
                if (flowPreview.flow.erc20[i].token == address(WMATIC)) {
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
            WMATIC.deposit{value: wrappedMaticAmount}();
        }
        // approve the flow contract
        WMATIC.approve(address(instance), wrappedMaticAmount);
        
        instance.flow(evaluable, callerContext, signedContexts);
    }
}