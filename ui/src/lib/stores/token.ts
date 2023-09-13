import { IFlowERC1155V3 } from "$lib/abi/IFlowERC1155V3";
import { IERC20 } from "$lib/abi/IERC20";
import { makeContractStore } from "svelte-wagmi-stores";
import type { Address } from "viem";
import addresses from "$lib/addresses.json";
import { NativeTokenFlowERC1155Caller } from "$lib/abi/NativeTokenFlowERC1155Caller";

export const paymentTokenAddress = "0x2Eb1D24aB0eC5FD0058ab5073F1EA2d8A59783E5";

export const nftContract = makeContractStore(IFlowERC1155V3, addresses.instance as Address)
export const paymentToken = makeContractStore(IERC20, paymentTokenAddress as Address)
export const nativeTokenFlowCaller = makeContractStore(NativeTokenFlowERC1155Caller, addresses.nativeTokenFlowCaller as Address)