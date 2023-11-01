<script lang="ts">
  import { ready } from "$lib/stores/ready";
  import { Progressbar, Spinner } from "flowbite-svelte";
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

  let feInitProgress = 0;
  let renderInitProgress = 0;

  onMount(async () => {
    $isHolo = ["true", "1", "t"].includes(
      import.meta.env.VITE_APP_IS_HOLO?.toLowerCase()
    );
    await initHapp();
    feInitProgress++;
    await initRenderMemory();
    await initWeb3Modal($happ.dnaProperties.chainId);
    feInitProgress++;
    setIsHotHolder(await $happ.getEvmAddress());
    feInitProgress++;
    await tick();
    await initNftStore(await $happ.dnaProperties.chainId);
    feInitProgress++;
    const appInfo = await $happ.client.appInfo();
    const dnaHash = encodeHashToBase64(
      appInfo.cell_info.fractal_tribute[0]?.provisioned.cell_id[0]
    );

    console.log("🚀 dnaHash", dnaHash);
  });

  const initRenderMemory = async () => {
    $happ.intializeMasks();
    $happ.client.on("signal", (signal) => {
      if (
        typeof signal.payload == "string" &&
        signal.payload.includes("progress: ")
      ) {
        const progress = signal.payload.split("progress: ")[1];
        renderInitProgress = parseInt(progress);
      }
    });
  };

  $: initProgress = feInitProgress + renderInitProgress;
  $: if (initProgress == 38) $ready = true;
  $: console.log(renderInitProgress);
</script>

<svg class="h-0 w-0" viewBox="0 0 0 0" xmlns="http://www.w3.org/2000/svg">
  {@html defs}
</svg>

<div class="min-w-screen min-h-screen p-4 container mx-auto">
  {#if $happ && $ready}
    {#if $isHolo}
      <HostedRoutes />
    {:else}
      <Routes />
    {/if}
  {:else}
    <div
      class="w-screen h-screen flex flex-col gap-y-6 items-center justify-center fixed inset-0"
    >
      <img class="w-96" src={logo} />
      <span class="font-semibold text-xl mt-8">Loading game...</span>
      <Progressbar
        class="w-96"
        progress={((initProgress / 38) * 100).toString()}
      />
    </div>
  {/if}
</div>

<Toasts {toasts} />
<AdminModal />
