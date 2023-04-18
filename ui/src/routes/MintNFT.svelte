<script lang="ts">
  import { connected, signerAddress } from "svelte-ethers-store";
  import { connectWallet } from "../lib/connect-wallet/";
  import CreatePayload from "../nft_payload/nft_payload/CreatePayload.svelte";
</script>

{#if !$connected}
  <button on:click={connectWallet}>Connect your wallet</button>
{:else}
  <CreatePayload />
{/if}
