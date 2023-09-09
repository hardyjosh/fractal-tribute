// SPDX-License-Identifier: CAL
pragma solidity ^0.8.18;

import 'rain.flow/IFlowERC1155V3';

contract NativeTokenFlowERC1155Caller {
    error ERC721TransferNotSupported();
    error ERC1155TransferNotSupported();
    error ERC20TransferNotSupported();

    address public wrappedMaticAddress;

    function constructor(address _wrappedMaticAddress) public {
        wrappedMaticAddress = _wrappedMaticAddress;
    }

    function flow(
        IFlowERC1155V3 contract, 
        Evaluable calldata evaluable,
        uint256[] calldata callerContext,
        SignedContextV1[] calldata signedContexts
    ) external payable public {
        FlowERC1155IOV1 memory data = contract.previewFlow(evaluable, callerContext, signedContexts);
        uint256 wrappedMaticAmount = 0

        if (data.flow.erc721.length > 0) {
            revert ERC721TransferNotSupported();
        }
        if (data.flow.erc1155.length > 0) {
            revert ERC1155TransferNotSupported();
        }
        if (data.flow.erc20.length > 0) {
            for (uint256 i = 0; i < data.flow.erc20.length; i++) {
                if (data.flow.erc20[i].token == wrappedMaticAddress && data.flow.erc20[i].to == address(this) ) {
                    wrappedMaticAmount += data.flow.erc20[i].amount;
                } else {
                    revert ERC20TransferNotSupported();
                }
            }
        }


    }
}