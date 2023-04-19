<script lang="ts">
  import { signer } from "svelte-ethers-store";
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
  import { mint } from "../../lib/mint/mint";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  enum TransactionStatus {
    None = "none",
    Pending = "pending",
    Success = "success",
    Failure = "failure",
  }

  let txStatus: TransactionStatus = TransactionStatus.None;

  let name: string = "";
  let description: string = "";

  let errorSnackbar: Snackbar;
  $: buttonDisabled = txStatus == TransactionStatus.Pending;

  async function createPayload() {
    const payload_bytes = encode({ name, description });
    const payloadEntry: Payload = {
      payload_bytes,
    };

    const hashedPayload = hexlify(keccak256(payload_bytes));

    try {
      const record: any = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "create_payload",
        payload: payloadEntry,
      });

      if (record) {
        txStatus = TransactionStatus.Pending;
        try {
          const receipt = await mint(hashedPayload, $signer);
          txStatus = TransactionStatus.Success;
          dispatch("payload-created");
        } catch (e) {
          console.log(e);
          errorSnackbar.labelText = `Error with transaction: ${e}`;
          txStatus = TransactionStatus.Failure;
        }
      }
    } catch (e) {
      console.log("response", e);
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
    disabled={buttonDisabled}
    raised
    label="Create Payload"
    on:click={createPayload}
  />
  {#if txStatus == TransactionStatus.Pending}
    <span>Transaction pending... check your wallet to confirm.</span>
  {:else if txStatus == TransactionStatus.Success}
    <span>Transaction successful!</span>
  {:else if txStatus == TransactionStatus.Failure}
    <span>Transaction failed!</span>
  {/if}
</div>
