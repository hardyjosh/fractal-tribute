import type { AppAgentClient } from "@holochain/client";
import type { EvmKeyBinding } from "../../nft_payload/nft_payload/types";
import { getContext } from "svelte";
import { clientContext } from "../../contexts";
import { decode } from "@msgpack/msgpack";
import { writable } from "svelte/store";
import { hexlify } from "@rainprotocol/rainlang";


export const getAgentEvmKey = async (client: AppAgentClient): Promise<{ evmKey: Uint8Array, error }> => {
    let evmKey: Uint8Array | null
    let error

    try {
        const evmKey = await client.callZome({
            cap_secret: null,
            role_name: "nft_payload",
            zome_name: "nft_payload",
            fn_name: "get_evm_address",
            payload: null,
        });
        return { evmKey, error };
    } catch (error) {
        console.log(error.data.data);
        return { evmKey, error }
    }
}