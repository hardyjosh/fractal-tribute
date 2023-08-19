<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from "svelte";
  import "@material/mwc-circular-progress";
  import { decode } from "@msgpack/msgpack";
  import type {
    Record,
    ActionHash,
    AppAgentClient,
    EntryHash,
    AgentPubKey,
    DnaHash,
  } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { EvmKeyBinding, Payload } from "./types";
  import "@material/mwc-circular-progress";
  import type { Snackbar } from "@material/mwc-snackbar";
  import "@material/mwc-snackbar";
  import "@material/mwc-icon-button";

  const dispatch = createEventDispatcher();

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  let loading = true;
  let error: any = undefined;

  let record: any;
  let key;

  $: error, loading, record, key;

  async function fetchPayload() {
    loading = true;
    error = undefined;
    record = undefined;

    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "get_evm_address",
        payload: null,
      });

      key = record as Uint8Array;

      if (record) {
        console.log(record);
        key = decode((record.entry as any).Present.entry) as EvmKeyBinding;
        console.log(key);
      }
    } catch (e) {
      console.log(e);
      error = e;
    }

    loading = false;
  }
</script>

<button on:click={fetchPayload}>Fetch current agent's EVM key</button>
