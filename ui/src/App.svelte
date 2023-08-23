<script lang="ts">
  import "./app.postcss";
  import WalletContext from "$lib/contexts/WalletContext.svelte";
  import Routes from "$routes/Routes.svelte";
  import { onMount } from "svelte";
  import { AppAgentWebsocket } from "@holochain/client";
  import { initHapp, happ, initWeb3Modal } from "$lib/stores";

  let client: AppAgentWebsocket;

  onMount(async () => {
    client = await AppAgentWebsocket.connect("", "fractal_tribute");
    initHapp(client);
    initWeb3Modal();
  });
</script>

{#if $happ}
  <Routes />
{/if}
