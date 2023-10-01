<script lang="ts">
  import "./app.postcss";
  import Routes from "$routes/Routes.svelte";
  import { onMount, tick } from "svelte";
  import { initHapp, happ, initWeb3Modal } from "$lib/stores";
  import { toasts } from "$lib/components/toasts";
  import Toasts from "$lib/components/toasts/Toasts.svelte";
  import RandomGameMoves from "$lib/components/RandomGameMoves.svelte";
  import defs from "../../dnas/fractal_tribute/zomes/integrity/fractal_tribute/src/defs.svg?raw";
  import { encodeHashToBase64 } from "@holochain/client";
  import AdminModal from "$lib/components/AdminModal.svelte";

  let ready = false;

  onMount(async () => {
    await initHapp();
    await initWeb3Modal($happ.dnaProperties.chainId);
    await tick();
    ready = true;
    const appInfo = await $happ.client.appInfo();
    const dnaHash = encodeHashToBase64(
      appInfo.cell_info.fractal_tribute[0]?.provisioned.cell_id[0]
    );

    console.log("🚀 dnaHash", dnaHash);
  });
</script>

<svg class="h-0 w-0" viewBox="0 0 0 0" xmlns="http://www.w3.org/2000/svg">
  {@html defs}
</svg>

<div class="min-w-screen min-h-screen p-4 container mx-auto">
  {#if $happ && ready}
    <Routes />
  {/if}
</div>

<Toasts {toasts} />
<AdminModal />
