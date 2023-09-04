<script lang="ts">
  import logo from "$lib/assets/logo.svg";
  import { currentRoute, routes, setRoute } from "$lib/stores/routes";
</script>

<div class="flex gap-x-2 mb-4 items-center">
  <img src={logo} alt="fractal tribute logo" />
  {#each routes as route, i}
    <button
      on:click={() => {
        setRoute(route.name);
      }}>{route.name}</button
    >
  {/each}
</div>

<svelte:component this={$currentRoute.component} />
