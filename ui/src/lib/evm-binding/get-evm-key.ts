import type { AppAgentClient } from "@holochain/client";
import type { EvmKeyBinding } from "../../nft_payload/nft_payload/types";
import { getContext } from "svelte";
import { clientContext } from "../../contexts";
import { decode } from "@msgpack/msgpack";
import { writable } from "svelte/store";

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

export const getAgentEvmKey = async (): Promise<{ evmKeyBinding: EvmKeyBinding, error }> => {
    let evmKeyBinding: EvmKeyBinding | null
    let error

    try {
        const record = await client.callZome({
            cap_secret: null,
            role_name: "nft_payload",
            zome_name: "nft_payload",
            fn_name: "get_evm_address",
            payload: null,
        });

        if (record) {
            console.log(record);
            const evmKeyBinding = decode((record.entry as any).Present.entry) as EvmKeyBinding;
            return { evmKeyBinding, error };
        }
    } catch (error) {
        console.log(error);
        return { evmKeyBinding, error }
    }
}

export const agentEvmKey = writable(await getAgentEvmKey());