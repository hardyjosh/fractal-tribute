<script lang="ts">
  import { web3modal } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import Gallery from "./Gallery.svelte";
  import Home from "./Home.svelte";

  type Route = {
    name: string;
    component: typeof Home | typeof Gallery;
  };

  const routes: Route[] = [
    {
      name: "Home",
      component: Home,
    },
    {
      name: "Gallery",
      component: Gallery,
    },
  ];

  let currentRoute: Route = routes[0];
</script>

<div class="flex gap-x-2">
  {#each routes as route, i}
    <span
      on:click={() => {
        currentRoute = routes[i];
      }}>{route.name}</span
    >
  {/each}
  <Button on:click={$web3modal.openModal}>Connect wallet</Button>
</div>

<svelte:component this={currentRoute.component} />
