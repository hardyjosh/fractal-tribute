<script lang="ts">
  import BrowseNFTs from "./routes/BrowseNFTs.svelte";
  import MintNFT from "./routes/MintNFT.svelte";
  import { onMount, setContext } from "svelte";
  import type { AppAgentClient } from "@holochain/client";
  import { AppAgentWebsocket } from "@holochain/client";
  import "@material/mwc-circular-progress";

  import { clientContext } from "./contexts";
  import ConnectWallet from "./lib/connect-wallet/ConnectWallet.svelte";

  let client: AppAgentClient | undefined;
  let loading = true;

  enum Routes {
    MintNFT = "mint-nft",
    ViewNFTs = "view-nfts",
  }

  let activeRoute = Routes.MintNFT;

  $: client, loading;

  onMount(async () => {
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect("", "nft-payload");
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });
</script>

<main style="display: flex; flex-direction: column; flex: 1; ">
  {#if loading}
    <div
      style="display: flex; flex: 1; align-items: center; justify-content: center"
    >
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <div
      style="display: flex; flex-direction: row; justify-content: space-between; margin-bottom: 1em; border-bottom: 1px solid black;"
    >
      <div style="display: flex; flex-direction: row;">
        <span style="margin-right:20px;">Holochain/EVM POC</span>
        <div>
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
      </div>
      <ConnectWallet />
    </div>
    {#if activeRoute == Routes.MintNFT}
      <MintNFT />
    {:else if activeRoute == Routes.ViewNFTs}
      <BrowseNFTs />
    {/if}
  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
