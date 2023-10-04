<script lang="ts">
  import { happ } from "$lib/stores";
  import { onMount, setContext } from "svelte";
  import { createCountdownStore } from "$lib/stores/countdown";
  import logo from "$lib/assets/logo.svg";
  import { Button, Modal } from "flowbite-svelte";
  import { countdownContext } from "$lib/contexts";
  import HowToPlay from "$lib/components/HowToPlay.svelte";
  import { ADDITIONAL_MINT_PERIOD } from "$lib/constants";
  import { writable, type Writable } from "svelte/store";
  import type { ParticipationProof } from "$lib/types";
  import Marketplace from "$routes/hosted/Marketplace.svelte";
  import Claims from "$routes/hosted/Claims.svelte";

  const countdown = createCountdownStore($happ.dnaProperties.gameEndTime);
  const snapshotEndCountdown = createCountdownStore(
    new Date($happ.dnaProperties.gameEndTime.getTime() + ADDITIONAL_MINT_PERIOD)
  );

  setContext(countdownContext, { countdown, snapshotEndCountdown });

  let open: boolean = false;

  type Route = (typeof routes)[number];
  type RouteName = Route["name"];

  export const routes = [
    {
      name: "Marketplace",
      component: Marketplace,
    },
    {
      name: "Claims",
      component: Claims,
    },
  ] as const;

  export const currentRoute: Writable<Route> = writable(routes[0]);

  export const setRoute = (name: RouteName) => {
    const newRoute = routes.find((r) => r.name === name);
    if (newRoute) currentRoute.set(newRoute);
  };

  const participations = writable<ParticipationProof>();
  setContext("participations", participations);

  onMount(async () => {
    $participations = await $happ.buildAgentParticipation();
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

    <Button
      size="lg"
      href="https://github.com/holochain-apps/fractal-tribute-kangaroo/releases/tag/v0.1.0"
      class="bg-fractalorange border-2 border-black">Download for Mac</Button
    >
    <Button
      size="lg"
      href="https://github.com/holochain-apps/fractal-tribute-kangaroo/releases/tag/v0.1.0"
      class="bg-fractalorange border-2 border-black"
      >Download for Windows</Button
    >
    <!-- <Button
      on:click={() => {
        open = true;
      }}
      size="lg"
      color="none"
      class="border-2 border-black"
    >
      <QuestionCircleOutline class="mr-2" />
      How to play
    </Button> -->
  </div>
</div>

<svelte:component this={$currentRoute.component} />

<Modal bind:open defaultClass="rounded-2xl">
  <HowToPlay
    hasEvmAddress={true}
    on:onboarding-complete={() => {
      open = false;
    }}
  /></Modal
>
