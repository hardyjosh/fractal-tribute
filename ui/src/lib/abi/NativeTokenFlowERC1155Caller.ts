export const NativeTokenFlowERC1155Caller = [
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "_wmatic",
                "type": "address"
            }
        ],
        "stateMutability": "nonpayable",
        "type": "constructor"
    },
    {
        "inputs": [],
        "name": "CantBeTokenRecipient",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "ERC1155TransferNotSupported",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "ERC20TransferNotSupported",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "ERC721TransferNotSupported",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "WrongValueSent",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "WMATIC",
        "outputs": [
            {
                "internalType": "contract IWETH",
                "name": "",
                "type": "address"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "contract IFlowERC1155V3",
                "name": "instance",
                "type": "address"
            },
            {
                "components": [
                    {
                        "internalType": "contract IInterpreterV1",
                        "name": "interpreter",
                        "type": "address"
                    },
                    {
                        "internalType": "contract IInterpreterStoreV1",
                        "name": "store",
                        "type": "address"
                    },
                    {
                        "internalType": "address",
                        "name": "expression",
                        "type": "address"
                    }
                ],
                "internalType": "struct Evaluable",
                "name": "evaluable",
                "type": "tuple"
            },
            {
                "internalType": "uint256[]",
                "name": "callerContext",
                "type": "uint256[]"
            },
            {
                "components": [
                    {
                        "internalType": "address",
                        "name": "signer",
                        "type": "address"
                    },
                    {
                        "internalType": "uint256[]",
                        "name": "context",
                        "type": "uint256[]"
                    },
                    {
                        "internalType": "bytes",
                        "name": "signature",
                        "type": "bytes"
                    }
                ],
                "internalType": "struct SignedContextV1[]",
                "name": "signedContexts",
                "type": "tuple[]"
            }
        ],
        "name": "flow",
        "outputs": [],
        "stateMutability": "payable",
        "type": "function"
    }
] as const;