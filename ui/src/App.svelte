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

  import defs from "../../dnas/fractal_tribute/zomes/integrity/fractal_tribute/src/defs.svg?raw";

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

<svg class="h-0 w-0" viewBox="0 0 0 0" xmlns="http://www.w3.org/2000/svg">
  {@html defs}
</svg>
<div class="min-w-screen min-h-screen p-4 container mx-auto">
  {#if $happ && ready}
    <Routes />
  {/if}
</div>
<RandomGameMoves />

<Toasts {toasts} />
