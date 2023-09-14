<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { happ, web3modal } from "$lib/stores/";
  import { encodeHashToBase64, type Record } from "@holochain/client";
  import type { EvmKeyBinding } from "$lib/types";
  import { Button } from "flowbite-svelte";
  import { formatAddress } from "$lib/helpers";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { toBytes, toHex } from "viem";
  import { addToast } from "$lib/components/toasts";
  import WalletSvg from "$lib/assets/wallet.svg";
  import binding from "$lib/assets/binding.svg";

  const dispatch = createEventDispatcher();

  enum EvmKeyBindingStatus {
    NotCreated,
    AwaitingSignature,
    Created,
  }

  $: myPubKey = $happ.myPubKey();

  let evmKeyBindingStatus: EvmKeyBindingStatus = EvmKeyBindingStatus.NotCreated;

  async function createEvmKeyBinding() {
    try {
      evmKeyBindingStatus = EvmKeyBindingStatus.AwaitingSignature;
      const sig = await $walletClient.signMessage({
        account: $account.address,
        message: { raw: toHex(myPubKey) },
      });

      const evmKeyBindingEntry: EvmKeyBinding = {
        evm_key: toBytes($account.address),
        signature_bytes: toBytes(sig),
      };

      const resp = await $happ.createEvmKeyBinding(evmKeyBindingEntry);
      evmKeyBindingStatus = EvmKeyBindingStatus.Created;
      addToast("success", "Binding successfully created!");
      dispatch("evmKeyBindingCreated");
    } catch (e) {
      console.log(e?.details?.message);
      evmKeyBindingStatus = EvmKeyBindingStatus.NotCreated;
      addToast(
        "error",
        `Error creating the evm key binding: ${
          e?.data?.data || e?.shortMessage || e
        }`
      );
    }
  }
</script>

<div
  class="flex flex-col justify-center items-center rounded-lg border-black bg-primary-25 h-full max-w-full"
>
  <div class="gap-y-6 w-full flex flex-col break-words p-4">
    {#if $account?.isConnected}
      {#if evmKeyBindingStatus === EvmKeyBindingStatus.NotCreated}
        <div class="flex flex-col gap-y-2 items-start">
          <div class="text-2xl font-semibold mb-4">You are binding</div>
          <div>EVM Key</div>
          <div
            class="bg-white text-gray-800 border-black border-2 rounded-lg px-4 py-2 self-stretch"
          >
            {$account.address}
          </div>
          <div
            class="self-stretch flex justify-center items-center w-full relative gap-x-2"
          >
            <div class="border-t border-gray-300 w-full" />
            <img src={binding} class="w-8" alt="binding icon" />
            <div class="border-t border-gray-300 w-full" />
          </div>
          <div>Holochain Key</div>
          <div
            class="bg-white text-gray-800 border-black border-2 rounded-lg px-4 py-2 self-stretch"
          >
            {#if myPubKey}
              {encodeHashToBase64(myPubKey)}
            {/if}
          </div>
        </div>
        <Button
          class="bg-fractalorange border-2 border-black"
          disabled={!$account.isConnected}
          on:click={() => createEvmKeyBinding()}>Bind wallet</Button
        >
      {:else if evmKeyBindingStatus === EvmKeyBindingStatus.AwaitingSignature}
        <div class="flex flex-col items-center">
          <p>Please check your wallet and sign your Holochain agent key</p>
        </div>
      {/if}
    {:else}
      <div class="gap-y-4 flex flex-col justify-center items-center">
        <img src={WalletSvg} class="w-32" alt="wallet icon" />
        <span>Connect your wallet to play.</span>
        <Button
          class="bg-fractalorange border-2 border-black"
          on:click={() => {
            $web3modal.openModal();
          }}>Connect wallet</Button
        >
      </div>
    {/if}
  </div>
</div>
