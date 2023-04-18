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
  import type { Payload } from "./types";
  import "@material/mwc-button";
  import "@material/mwc-snackbar";
  import type { Snackbar } from "@material/mwc-snackbar";
  import "@material/mwc-textarea";

  import "@material/mwc-textfield";
  import { hexlify, keccak256 } from "ethers/lib/utils";
  import { decode, encode } from "@msgpack/msgpack";
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let name: string = "";
  let description: string = "";

  let errorSnackbar: Snackbar;

  $: name, description;

  async function createPayload() {
    const payload_bytes = encode({ name, description });
    const payloadEntry: Payload = {
      payload_bytes,
    };

    console.log("payload bytes", payload_bytes);
    console.log(decode(payload_bytes));
    console.log("payload bytes as hex", hexlify(payload_bytes));
    console.log("hashed payload bytes", hexlify(keccak256(payload_bytes)));

    try {
      const record: any = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "create_payload",
        payload: payloadEntry,
      });

      // dispatch("payload-created", {
      //   payloadHash: record.signed_action.hashed.hash,
      // });
      // console.log(record);
      console.log("content hash", hexlify(record));
      // console.log(record.signed_action.hashed.hash);
      // console.log(record.entry);
    } catch (e) {
      console.log("response", e);
      // errorSnackbar.labelText = `Error creating the payload: ${e.data.data}`;
      // errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Payload</span>

  <div style="margin-bottom: 16px">
    <mwc-textfield
      outlined
      label="Name"
      value={name}
      on:input={(e) => {
        name = e.target.value;
      }}
      required
    />
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textarea
      outlined
      label="Description"
      value={description}
      on:input={(e) => {
        description = e.target.value;
      }}
      required
    />
  </div>

  <mwc-button raised label="Create Payload" on:click={() => createPayload()} />
</div>
