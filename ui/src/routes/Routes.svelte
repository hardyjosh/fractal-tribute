<script lang="ts">
  import logo from "$lib/assets/logo.svg";
  import { currentRoute, routes, setRoute } from "$lib/stores/routes";
  import { Button } from "flowbite-svelte";
  import { QuestionCircleOutline } from "flowbite-svelte-icons";
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
    <Button size="lg" color="none" class="border-2 border-black">
      <QuestionCircleOutline class="mr-2" />
      How to play
    </Button>
  </div>
</div>

<svelte:component this={$currentRoute.component} />
