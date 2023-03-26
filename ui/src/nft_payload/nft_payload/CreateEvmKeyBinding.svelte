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

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  // export let creator!: AgentPubKey;

  let evmKey: number = 0;

  let errorSnackbar: Snackbar;

  $: evmKey;
  $: isEvmKeyBindingValid = true && true;

  async function createEvmKeyBinding() {
    const evmKeyBindingEntry: EvmKeyBinding = {
      evm_key: evmKey!,
      // creator: creator!,
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
      errorSnackbar.labelText = `Error creating the evm key binding: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create EvmKeyBinding</span>

  <div style="margin-bottom: 16px">
    <div style="display: flex; flex-direction: row">
      <span style="margin-right: 4px">Evm Key</span>
      <input bind:value={evmKey} type="number" />
      <!-- <mwc-slider value={ evmKey } on:input={e => { evmKey = e.detail.value; } } discrete></mwc-slider> -->
    </div>
  </div>

  <mwc-button
    raised
    label="Create EvmKeyBinding"
    disabled={!isEvmKeyBindingValid}
    on:click={() => createEvmKeyBinding()}
  />
</div>
