<script lang="ts">
  import { connected } from "svelte-ethers-store";
  import { connectWallet } from "../lib/connect-wallet/";
  import CreatePayload from "../nft_payload/nft_payload/CreatePayload.svelte";
  import CreateEvmKeyBinding from "../nft_payload/nft_payload/CreateEvmKeyBinding.svelte";
  import { onMount } from "svelte";
  import { agentEvmKey } from "../lib/evm-binding/get-evm-key";

  $: console.log("agent key", $agentEvmKey);
</script>

{#if !$connected}
  <button on:click={connectWallet}>Connect your wallet</button>
{:else}
  <CreateEvmKeyBinding />
  <CreatePayload />
{/if}
