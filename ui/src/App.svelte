<script lang="ts">
  import BrowseNFTs from "./routes/BrowseNFTs.svelte";
  import MintNFT from "./routes/MintNFT.svelte";
  import { onMount, setContext } from "svelte";
  import type { ActionHash, AppAgentClient } from "@holochain/client";
  import { AppAgentWebsocket } from "@holochain/client";
  import "@material/mwc-circular-progress";

  import { clientContext } from "./contexts";
  import CreateEvmKeyBinding from "./nft_payload/nft_payload/CreateEvmKeyBinding.svelte";
  import CreatePayload from "./nft_payload/nft_payload/CreatePayload.svelte";
  import GetEvmKey from "./nft_payload/nft_payload/GetEvmKey.svelte";
  import ConnectWallet from "./lib/connect-wallet/ConnectWallet.svelte";
  import Deploy from "./lib/deploy/Deploy.svelte";
  import Mint from "./lib/mint/Mint.svelte";
  import GetOnchainNfTs from "./nft_payload/nft_payload/GetOnchainNFTs.svelte";

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

<main>
  {#if loading}
    <div
      style="display: flex; flex: 1; align-items: center; justify-content: center"
    >
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <div
        style="display: flex; flex-direction: row; justify-content: space-between;"
      >
        <div style="display: flex; flex-direction: row;">
          <span>Holochain/EVM POC</span>
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
    </div>
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
