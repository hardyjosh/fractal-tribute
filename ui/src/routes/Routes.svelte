<script lang="ts">
  import { happ } from "$lib/stores";
  import { onMount, setContext } from "svelte";
  import { createCountdownStore, formatCountdown } from "$lib/stores/countdown";
  import logo from "$lib/assets/logo.svg";
  import { currentRoute, routes, setRoute } from "$lib/stores/routes";
  import { Button, Modal } from "flowbite-svelte";
  import { QuestionCircleOutline } from "flowbite-svelte-icons";
  import { countdownContext } from "$lib/contexts";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import HowToPlay from "$lib/components/HowToPlay.svelte";

  let countdown = createCountdownStore($happ.dnaProperties.gameEndTime);
  setContext(countdownContext, countdown);

  let open: boolean = false;
  let evm_address = null;

  onMount(async () => {
    evm_address = await $happ.getEvmAddress();
    if (!evm_address) {
      open = true;
    }
  });
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
      <span>Game ends in: </span><span class="text-green-600 font-bold"
        >{formatCountdown($countdown)}</span
      >
    </div>
    <Button
      on:click={() => {
        open = true;
      }}
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

<Modal bind:open>
  <HowToPlay
    hasEvmAddress={!!evm_address}
    on:onboarding-complete={() => {
      open = false;
    }}
  /></Modal
>
