<script lang="ts">
  import { fade } from "svelte/transition";
  import { createEventDispatcher } from "svelte";
  import { happ, web3modal } from "$lib/stores/";
  import { encodeHashToBase64, type Record } from "@holochain/client";
  import type { EvmKeyBinding } from "$lib/types";
  import { Button, Heading, Spinner } from "flowbite-svelte";
  import { formatAddress } from "$lib/helpers";
  import { account, walletClient } from "svelte-wagmi-stores";
  import { toBytes, toHex } from "viem";
  import { addToast } from "$lib/components/toasts";
  import WalletSvg from "$lib/assets/wallet.svg";
  import binding from "$lib/assets/binding.svg";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

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

{#if $account?.isConnected}
  <div
    class="gap-y-6 w-full flex flex-col break-words h-full justify-center grow"
  >
    {#if evmKeyBindingStatus === EvmKeyBindingStatus.NotCreated}
      <div
        in:fade|global={{ duration: 200 }}
        class="flex flex-col gap-y-2 items-start"
      >
        <Heading tag="h4" class="text-center"
          ><En>You are binding</En><Tr>Bunları bağlıyorsun</Tr></Heading
        >
        <div><En>EVM Key</En><Tr>EVM anahtarı</Tr></div>
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
        <div><En>Holochain Key</En><Tr>Holochain anahtarı</Tr></div>
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
        on:click={() => createEvmKeyBinding()}
        ><En>Bind wallet</En><Tr>Cüzdanı bağla</Tr></Button
      >
    {:else if evmKeyBindingStatus === EvmKeyBindingStatus.AwaitingSignature}
      <div
        in:fade|global={{ duration: 200 }}
        class="flex flex-col items-center gap-y-6"
      >
        <Spinner size="10" />
        <p>
          <En>Please check your wallet and sign your Holochain agent key</En><Tr
            >Lütfen cüzdanınızı kontrol edin ve Holochain anahtarınızı imzalayın</Tr
          >
        </p>
      </div>
    {/if}
  </div>
{:else}
  <div
    in:fade|global={{ duration: 200 }}
    class="gap-y-4 flex flex-col items-center justify-center grow h-full -mt-8 text-center"
  >
    <Heading tag="h4" class="w-auto"
      ><En>Connect wallet</En><Tr>Cüzdan bağla</Tr></Heading
    >
    <img src={WalletSvg} class="w-32" alt="wallet icon" />
    <span>
      <En
        >To continue, please connect a wallet of your choice. Click the button
        below and scan the the QR code with your wallet app</En
      ><Tr
        >Devam etmek için, lütfen istediğin bir cüzdanı bağla. Aşağıdaki butona
        tıkla ve cihazındaki cüzdan uygulamasını kullanarak karekodu tara.</Tr
      ></span
    >
    <span
      ><En
        >Your wallet will need to be connected to Polygon to create and collect
        snapshots.</En
      ><Tr
        >Görüntü oluşturmak ve toplamak için cüzdanının Polygon ağına bağlı
        olması gerekir.</Tr
      ></span
    >
    <Button
      class="bg-fractalorange border-2 border-black"
      on:click={() => {
        $web3modal.openModal();
      }}><En>Connect wallet</En><Tr>Cüzdan bağla</Tr></Button
    >
  </div>
{/if}
