<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import type { AppAgentClient, Record } from "@holochain/client";
  import { clientContext } from "../../contexts";
  import type { EvmKeyBinding } from "./types";
  import "@material/mwc-button";
  import "@material/mwc-snackbar";
  import type { Snackbar } from "@material/mwc-snackbar";
  import "@material/mwc-slider";
  import { utils } from "ethers";
  import { signer, connected, signerAddress } from "svelte-ethers-store";
  import { hexlify } from "ethers/lib/utils";
  import { Alert, Button } from "flowbite-svelte";
  import { formatAddress } from "../../lib/utils";
  import { ArrowsUpDown, Ticket } from "svelte-heros-v2";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let evmKey: string = "";

  let errorSnackbar: Snackbar;

  $: evmKey;

  enum EvmKeyBindingStatus {
    NotCreated,
    AwaitingSignature,
    Created,
  }

  let evmKeyBindingStatus: EvmKeyBindingStatus = EvmKeyBindingStatus.NotCreated;

  async function createEvmKeyBinding() {
    evmKeyBindingStatus = EvmKeyBindingStatus.AwaitingSignature;
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
      evmKeyBindingStatus = EvmKeyBindingStatus.Created;
    } catch (e) {
      console.log(e);
      errorSnackbar.labelText = `Error creating the evm key binding: ${e.data.data}`;
      errorSnackbar.show();
    }
  }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading />

<div class="flex flex-col items-center justify-center pt-36">
  <div
    class="bg-white rounded-2xl p-6 max-w-md gap-y-6 items-center flex flex-col break-words"
  >
    <div class="flex flex-col gap-y-2 items-center">
      <div>You are binding EVM account</div>
      <div class="text-3xl text-gray-500 font-bold">
        {formatAddress($signerAddress)}
      </div>
      <ArrowsUpDown class="w-12 h-12 text-gray-500" />
      <div>with your Holochain agent key</div>
      <div class="text-3xl text-gray-500 font-bold">
        {formatAddress(hexlify(client.myPubKey))}
      </div>
    </div>
    <Alert color="green" class="text-lg"
      ><Ticket class="inline" />
      Membership proof obtained - you hold 1000 MintMe tokens which means you are
      eligible to join this happ.</Alert
    >
    <Button
      class="text-xl"
      disabled={!$connected}
      on:click={() => createEvmKeyBinding()}
      >Submit proof and bind EVM wallet to Holochain agent</Button
    >
    {#if evmKeyBindingStatus === EvmKeyBindingStatus.AwaitingSignature}
      <div class="text-blue-500">
        Please check your wallet and sign your Holochain agent key
      </div>
    {/if}
  </div>
</div>
