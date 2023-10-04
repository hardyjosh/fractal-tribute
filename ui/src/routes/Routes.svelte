<script lang="ts">
  import { happ } from "$lib/stores";
  import { onMount, setContext } from "svelte";
  import { createCountdownStore, formatCountdown } from "$lib/stores/countdown";
  import logo from "$lib/assets/logo.svg";
  import { currentRoute, routes, setRoute } from "$lib/stores/routes";
  import { Button, Modal } from "flowbite-svelte";
  import { QuestionCircleOutline } from "flowbite-svelte-icons";
  import { countdownContext } from "$lib/contexts";
  import HowToPlay from "$lib/components/HowToPlay.svelte";
  import { ADDITIONAL_MINT_PERIOD } from "$lib/constants";

  const countdown = createCountdownStore($happ.dnaProperties.gameEndTime);
  const snapshotEndCountdown = createCountdownStore(
    new Date($happ.dnaProperties.gameEndTime.getTime() + ADDITIONAL_MINT_PERIOD)
  );

  setContext(countdownContext, { countdown, snapshotEndCountdown });

  let open: boolean = false;
  let evm_address = null;

  onMount(async () => {
    evm_address = await $happ.getEvmAddress();
    if (!evm_address) {
      open = true;
    }
  });

  const openModal = async () => {
    evm_address = await $happ.getEvmAddress();
    open = true;
  };
</script>

<div class="flex gap-x-2 mb-4 items-center justify-between">
  <img src={logo} alt="fractal tribute logo" />
  <div class="flex items-center gap-x-6">
    {#each routes as route, i}
      <button
        class="text-lg"
        class:font-bold={$currentRoute.name === route.name}
        on:click={() => {
          setRoute(route.name);
        }}>{route.name}</button
      >
    {/each}
    <div class="border-2 rounded-lg p-3 self-stretch grow border-black">
      {#if $countdown?.timeRemaining}
        <span>Game ends in: </span><span class="text-green-600 font-bold"
          >{formatCountdown($countdown)}</span
        >
      {:else}
        <span>Game ended</span>
      {/if}
    </div>
    <Button
      on:click={openModal}
      size="lg"
      color="none"
      class="border-2 border-black"
    >
      <QuestionCircleOutline class="mr-2" />
      How to play
    </Button>
  </div>
</div>

<svelte:component this={$currentRoute.component} />

<Modal permanent={!evm_address} bind:open defaultClass="rounded-2xl">
  <HowToPlay
    hasEvmAddress={!!evm_address}
    on:onboarding-complete={() => {
      open = false;
    }}
  /></Modal
>
