<script lang="ts">
  import "./app.postcss";
  import WalletContext from "$lib/contexts/WalletContext.svelte";
  import Routes from "$routes/Routes.svelte";
  import { onMount, tick } from "svelte";
  import { AppAgentWebsocket } from "@holochain/client";
  import { initHapp, happ, initWeb3Modal } from "$lib/stores";
  import { toasts } from "$lib/components/toasts";
  import Toasts from "$lib/components/toasts/Toasts.svelte";

  let client: AppAgentWebsocket;
  let ready = false;

  onMount(async () => {
    client = await AppAgentWebsocket.connect("", "fractal_tribute");
    initHapp(client);
    initWeb3Modal();
    await tick();
    ready = true;
  });
</script>

<div class="min-w-screen min-h-screen p-4 container mx-auto">
  {#if $happ && ready}
    <Routes />
  {/if}
</div>

<Toasts {toasts} />
