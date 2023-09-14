import { IFlowERC1155V3 } from "$lib/abi/IFlowERC1155V3";
import { IERC20 } from "$lib/abi/IERC20";
import { makeContractStore } from "svelte-wagmi-stores";
import type { Address } from "viem";
import addresses from "$lib/addresses.json";
import { NativeTokenFlowERC1155Caller } from "$lib/abi/NativeTokenFlowERC1155Caller";

export const paymentTokenAddress = "0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889";

export const nftContract = makeContractStore(IFlowERC1155V3, addresses.instance as Address)
export const paymentToken = makeContractStore(IERC20, paymentTokenAddress as Address)
export const nativeTokenFlowCaller = makeContractStore(NativeTokenFlowERC1155Caller, addresses.nativeTokenFlowCaller as Address)