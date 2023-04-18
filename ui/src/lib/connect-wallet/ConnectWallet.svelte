<script lang="ts">
  import { connected, signerAddress } from "svelte-ethers-store";
  import { connectWallet } from "./";
</script>

{#if !$connected}
  <button on:click={connectWallet}>Connect Wallet</button>
{:else}
  <span>Connected: {$signerAddress}</span>
{/if}
