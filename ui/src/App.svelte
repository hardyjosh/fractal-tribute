<script lang="ts">
  import { walletClient, network } from "svelte-wagmi-stores";
  import "./app.postcss";
  import Routes from "$routes/Routes.svelte";
  import { onMount, tick } from "svelte";
  import { AppAgentWebsocket } from "@holochain/client";
  import { initHapp, happ, initWeb3Modal } from "$lib/stores";
  import { toasts } from "$lib/components/toasts";
  import Toasts from "$lib/components/toasts/Toasts.svelte";
  import { switchNetwork } from "@wagmi/core";
  import { polygonMumbai } from "viem/chains";
  import RandomGameMoves from "$lib/components/RandomGameMoves.svelte";

  let client: AppAgentWebsocket;
  let ready = false;

  onMount(async () => {
    client = await AppAgentWebsocket.connect("", "fractal_tribute");
    await initHapp(client);
    await initWeb3Modal($happ.dnaProperties.chainId);
    await tick();
    ready = true;
  });

  $: if (ready && $walletClient) {
    // $walletClient.addChain({ chain: polygonMumbai });
    // switchNetwork({ chainId: $happ.dnaProperties.chainId });
    console.log("should switch");
  }
</script>

<div class="min-w-screen min-h-screen p-4 container mx-auto">
  {#if $happ && ready}
    <!-- <RandomGameMoves /> -->
    <Routes />
  {/if}
</div>

<Toasts {toasts} />
