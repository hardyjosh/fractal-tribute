<script lang="ts">
  import "./app.postcss";
  import WalletContext from "$lib/contexts/WalletContext.svelte";
  import Routes from "$routes/Routes.svelte";
  import { onMount } from "svelte";
  import { AppAgentWebsocket } from "@holochain/client";
  import { DnaInterface, initHapp, happ } from "$lib/stores";

  let client: AppAgentWebsocket;
  let dnaInterface: DnaInterface;

  onMount(async () => {
    client = await AppAgentWebsocket.connect("", "fractal_tribute");
    initHapp(client);
  });
</script>

{#if $happ}
  <WalletContext>
    <Routes />
  </WalletContext>
{/if}
