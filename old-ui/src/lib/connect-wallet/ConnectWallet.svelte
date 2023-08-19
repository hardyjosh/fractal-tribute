<script lang="ts">
  import { connected, signerAddress } from "svelte-ethers-store";
  import { connectWallet } from "./";
  import { formatAddress } from "../utils";
</script>

{#if !$connected}
  <button
    class="px-4 py-3 rounded-full bg-white text-black text-xl border border-gray-400"
    on:click={connectWallet}
  >
    Connect Wallet
  </button>
{:else}
  <span class="text-base">Connected: {formatAddress($signerAddress)}</span>
{/if}
