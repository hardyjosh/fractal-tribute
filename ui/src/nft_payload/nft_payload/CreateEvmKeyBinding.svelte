<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import type {
    AppAgentClient,
    Record,
    EntryHash,
    AgentPubKey,
    ActionHash,
    DnaHash,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { EvmKeyBinding } from "./types";
  import "@material/mwc-button";
  import "@material/mwc-snackbar";
  import type { Snackbar } from "@material/mwc-snackbar";
  import "@material/mwc-slider";
  import { utils } from "ethers";
  import { signer, connected, signerAddress } from "svelte-ethers-store";
  import { arrayify, hexlify } from "ethers/lib/utils";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let evmKey: string = "";

  let errorSnackbar: Snackbar;

  $: evmKey;

  async function createEvmKeyBinding() {
    let sig = await $signer.signMessage(client.myPubKey);

    const evmKeyBindingEntry: EvmKeyBinding = {
      evm_key: utils.arrayify($signerAddress),
      signature_bytes: utils.arrayify(sig),
    };

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "create_evm_key_binding",
        payload: evmKeyBindingEntry,
      });
      dispatch("evm-key-binding-created", {
        evmKeyBindingHash: record.signed_action.hashed.hash,
      });
    } catch (e) {
      console.log(e);
      errorSnackbar.labelText = `Error creating the evm key binding: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />

<div style="display: flex; flex-direction: column">
  <div>You are linking EVM account {$signerAddress}</div>
  <div>to Holochain agent {hexlify(client.myPubKey)}</div>
  <mwc-button
    raised
    label="Link EVM wallet to Holochain agent"
    disabled={!$connected}
    on:click={() => createEvmKeyBinding()}
  />
</div>
