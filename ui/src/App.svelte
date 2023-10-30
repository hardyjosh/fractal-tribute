<script lang="ts">
  import { Spinner } from "flowbite-svelte";
  import { isHolo } from "$lib/stores";
  import "./app.postcss";
  import Routes from "$routes/Routes.svelte";
  import { onMount, tick } from "svelte";
  import { initHapp, happ, initWeb3Modal } from "$lib/stores";
  import { toasts } from "$lib/components/toasts";
  import Toasts from "$lib/components/toasts/Toasts.svelte";
  import defs from "../../dnas/fractal_tribute/zomes/integrity/fractal_tribute/src/defs.svg?raw";
  import { encodeHashToBase64 } from "@holochain/client";
  import AdminModal from "$lib/components/AdminModal.svelte";
  import HostedRoutes from "$routes/HostedRoutes.svelte";
  import logo from "$lib/assets/logo.svg";
  import { initNftStore } from "$lib/stores/nfts";
  import { setIsHotHolder } from "$lib/stores/hotHolder";

  let ready = false;

  onMount(async () => {
    $isHolo = ["true", "1", "t"].includes(
      import.meta.env.VITE_APP_IS_HOLO?.toLowerCase()
    );
    await initHapp();
    await initWeb3Modal($happ.dnaProperties.chainId);
    setIsHotHolder(await $happ.getEvmAddress());
    await tick();
    await initNftStore(await $happ.dnaProperties.chainId);
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
    {#if $isHolo}
      <HostedRoutes />
    {:else}
      <Routes />
    {/if}
  {:else if $isHolo}
    <div
      class="w-screen h-screen flex flex-col gap-y-4 items-center justify-center fixed inset-0"
    >
      <img class="w-96" src={logo} />
      <Spinner />
    </div>
  {/if}
</div>

<Toasts {toasts} />
<AdminModal />
