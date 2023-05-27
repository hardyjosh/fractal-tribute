<script lang="ts">
  import ConnectWallet from "./connect-wallet/ConnectWallet.svelte";
  import { Routes } from "./types";

  export let activeRoute: Routes;
</script>

<div
  class="flex flex-row justify-between items-center py-4 sticky top-4 mb-4 text-white font-bold text-2xl"
>
  <div class="flex flex-row gap-x-4">
    <button
      on:click={() => {
        activeRoute = Routes.Home;
      }}
      class="logo mr-4 text-5xl">holomint</button
    >
    <button
      on:click={() => {
        activeRoute = Routes.MintNFT;
      }}>Mint an NFT</button
    >
    <button
      on:click={() => {
        activeRoute = Routes.ViewNFTs;
      }}>View all NFTs</button
    >
  </div>
  <ConnectWallet />
</div>
