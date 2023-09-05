<script lang="ts">
  import { setContext } from "svelte";
  import { createCountdownStore, formatCountdown } from "$lib/stores/countdown";
  import logo from "$lib/assets/logo.svg";
  import { currentRoute, routes, setRoute } from "$lib/stores/routes";
  import { Button, Modal } from "flowbite-svelte";
  import { QuestionCircleOutline } from "flowbite-svelte-icons";
  import { countdownContext } from "$lib/contexts";

  let countdown = createCountdownStore(1696462866000);
  setContext(countdownContext, countdown);

  let open: boolean = false;
</script>

<div class="flex gap-x-2 mb-4 items-center justify-between">
  <img src={logo} alt="fractal tribute logo" />
  <div class="flex items-center gap-x-6">
    {#each routes as route, i}
      <button
        on:click={() => {
          setRoute(route.name);
        }}>{route.name}</button
      >
    {/each}
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
    <div>{formatCountdown($countdown)}</div>
  </div>
</div>

<svelte:component this={$currentRoute.component} />

<Modal bind:open>
  <p>How to play instructions go here :)</p>
</Modal>
