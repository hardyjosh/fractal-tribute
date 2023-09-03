<script lang="ts">
  import { web3modal } from "$lib/stores";
  import { Button } from "flowbite-svelte";
  import Gallery from "./Gallery.svelte";
  import Home from "./Home.svelte";
  import logo from "$lib/assets/logo.svg";

  type Route = {
    name: string;
    component: typeof Home | typeof Gallery;
  };

  const routes: Route[] = [
    {
      name: "Play",
      component: Home,
    },
    {
      name: "Gallery",
      component: Gallery,
    },
  ];

  let currentRoute: Route = routes[0];
</script>

<div class="flex gap-x-2 mb-4 items-center">
  <img src={logo} alt="fractal tribute logo" />
  {#each routes as route, i}
    <button
      on:click={() => {
        currentRoute = routes[i];
      }}>{route.name}</button
    >
  {/each}
  <!-- <Button on:click={$web3modal.openModal}>Connect wallet</Button> -->
</div>

<svelte:component this={currentRoute.component} />
