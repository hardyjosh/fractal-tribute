<script lang="ts">
  import { happ } from "$lib/stores";
  import { onMount, setContext } from "svelte";
  import { createCountdownStore, formatCountdown } from "$lib/stores/countdown";
  import logo from "$lib/assets/logo.svg";
  import { Button, Modal } from "flowbite-svelte";
  import { QuestionCircleOutline } from "flowbite-svelte-icons";
  import { countdownContext } from "$lib/contexts";
  import HowToPlay from "$lib/components/HowToPlay.svelte";
  import { ADDITIONAL_MINT_PERIOD } from "$lib/constants";
  import { writable, type Writable } from "svelte/store";
  import Home from "$routes/hosted/Home.svelte";
  import Snapshots from "$routes/hosted/Snapshots.svelte";
  import Contributors from "$routes/hosted/Contributors.svelte";
  import Participation from "$lib/components/Participation.svelte";
  import type { ParticipationProof } from "$lib/types";

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
      name: "Play",
      component: Home,
    },
    {
      name: "Snapshots",
      component: Snapshots,
    },
    {
      name: "Contributors",
      component: Contributors,
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

<Modal bind:open defaultClass="rounded-2xl">
  <HowToPlay
    hasEvmAddress={true}
    on:onboarding-complete={() => {
      open = false;
    }}
  /></Modal
>
