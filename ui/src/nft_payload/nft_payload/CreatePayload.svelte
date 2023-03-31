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
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let name: string = "";
  let description: string = "";

  let errorSnackbar: Snackbar;

  $: name, description;
  $: isPayloadValid = true && name !== "" && description !== "";

  async function createPayload() {
    const payloadEntry: Payload = {
      name: name!,
      description: description!,
    };

    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "create_payload",
        payload: payloadEntry,
      });
      dispatch("payload-created", {
        payloadHash: record.signed_action.hashed.hash,
      });
      console.log(record.signed_action.hashed.hash);
    } catch (e) {
      errorSnackbar.labelText = `Error creating the payload: ${e.data.data}`;
      errorSnackbar.show();
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

  <mwc-button
    raised
    label="Create Payload"
    disabled={!isPayloadValid}
    on:click={() => createPayload()}
  />
</div>
