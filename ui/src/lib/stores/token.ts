import { IFlowERC1155V3 } from "$lib/abi/IFlowERC1155V3";
import { IERC20 } from "$lib/abi/IERC20";
import { makeContractStore } from "svelte-wagmi-stores";
import type { Address } from "viem";
import addresses from "$lib/addresses.json";
import { NativeTokenFlowERC1155Caller } from "$lib/abi/NativeTokenFlowERC1155Caller";
import { derived, type Readable } from "svelte/store";
import { happ } from "$lib/stores";

export const paymentTokenAddress: Readable<Address> = derived(happ, ($happ, set) => {
    if ($happ) {
        set($happ.dnaProperties.paymentTokenAddress)
    }
})

export const paymentToken: ReturnType<typeof makeContractStore<typeof IERC20>> = derived(paymentTokenAddress, ($paymentTokenAddress, set) => {
    if ($paymentTokenAddress) {
        const paymentToken = makeContractStore(IERC20, $paymentTokenAddress as Address)
        paymentToken.subscribe(set)
    }
})

export const nftContract = makeContractStore(IFlowERC1155V3, addresses.instance as Address)
export const nativeTokenFlowCaller = makeContractStore(NativeTokenFlowERC1155Caller, addresses.nativeTokenFlowCaller as Address)
