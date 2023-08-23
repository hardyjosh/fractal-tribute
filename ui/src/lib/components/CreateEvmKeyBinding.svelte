<script lang="ts">
  import { happ } from "$lib/stores/";
  import type { Record } from "@holochain/client";
  import type { EvmKeyBinding } from "$lib/types";
  import { Button } from "flowbite-svelte";
  import { formatAddress } from "$lib/helpers";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { toBytes, toHex } from "viem";
  import { addSnackBar } from "$lib/components/snackbar";

  enum EvmKeyBindingStatus {
    NotCreated,
    AwaitingSignature,
    Created,
  }

  $: myPubKey = $happ.myPubKey();

  let evmKeyBindingStatus: EvmKeyBindingStatus = EvmKeyBindingStatus.NotCreated;

  async function createEvmKeyBinding() {
    evmKeyBindingStatus = EvmKeyBindingStatus.AwaitingSignature;
    const sig = await $walletClient.signMessage({
      account: $account.address,
      message: { raw: toHex(myPubKey) },
    });

    const evmKeyBindingEntry: EvmKeyBinding = {
      evm_key: toBytes($account.address),
      signature_bytes: toBytes(sig),
    };

    try {
      await $happ.createEvmKeyBinding(evmKeyBindingEntry);
      evmKeyBindingStatus = EvmKeyBindingStatus.Created;
    } catch (e) {
      console.log(e);
      addSnackBar(`Error creating the evm key binding: ${e?.data?.data || e}`);
    }
  }
</script>

{#if $account?.isConnected}
  <div class="flex flex-col items-center justify-center">
    <div
      class="bg-white rounded-2xl p-6 max-w-md gap-y-6 items-center flex flex-col break-words"
    >
      <div class="flex flex-col gap-y-2 items-center">
        <div>You are binding EVM account</div>
        <div class="text-3xl text-gray-500 font-bold">
          {formatAddress($account.address)}
        </div>
        <!-- <IconOutline name="arrows-up-down" class="w-12 h-12 text-gray-500" /> -->
        <div>with your Holochain agent key</div>
        <div class="text-3xl text-gray-500 font-bold">
          {#if myPubKey}
            {formatAddress(toHex(myPubKey))}
          {/if}
        </div>
      </div>
      <Button
        class="text-xl"
        disabled={!$account.isConnected}
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
{/if}
