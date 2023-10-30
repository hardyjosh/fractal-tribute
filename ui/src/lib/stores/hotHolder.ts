import { HOT_ADDRESS, HOT_TOKEN_THRESHOLD } from "$lib/constants";
import { fetchBalance } from "@wagmi/core";
import { writable } from "svelte/store";
import type { Address } from "viem";

export const isHotHolder = writable(false);

export const setIsHotHolder = async (account: Address) => {
    if (!account) {
        return
    }
    const balance = await fetchBalance({
        address: account,
        token: HOT_ADDRESS,
        chainId: 1,
    })
    isHotHolder.set(balance.value > HOT_TOKEN_THRESHOLD)
}