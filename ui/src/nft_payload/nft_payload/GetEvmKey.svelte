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
  import { concat, hexlify } from "ethers/lib/utils";

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
      console.log("clicked");
      record = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "get_evm_address",
        payload: null,
      });
      console.log("returned evm byte", record);
      key = record as Uint8Array;
      const content = concat([key, Uint8Array.from([1, 2, 3, 4])]);
      console.log("key and content as hex", hexlify(content));
      // if (record) {
      //   key = decode((record.entry as any).Present.entry) as EvmKeyBinding;
      //   console.log(key);
      // }
    } catch (e) {
      console.log(e);
      error = e;
    }

    loading = false;
  }
</script>

<button on:click={fetchPayload}>get it yeah</button>
