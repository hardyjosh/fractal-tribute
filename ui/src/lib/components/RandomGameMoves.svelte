<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { onMount, onDestroy } from "svelte";
  import { generateRandomGameMove } from "$lib/helpers";
  import { happ } from "$lib/stores";

  let interval;
  let getMovesInterval;
  let numOfMoves;

  const startMakingRandomMoves = () => {
    interval = setInterval(() => {
      $happ.createGameMove(generateRandomGameMove());
    }, 150);
  };

  const stopMakingRandomMoves = () => {
    clearInterval(interval);
    interval = null;
  };

  const getMoves = async () => {
    console.log("getting moves");
    numOfMoves = await $happ.getNumberOfMoves();
    console.log("got moves");
  };

  onMount(() => {
    getMovesInterval = setInterval(getMoves, 20000);
  });

  onDestroy(() => {
    clearInterval(getMovesInterval);
  });

  $: if ($happ) getMoves();
</script>

<div class="flex flex-col gap-y-2 max-w-sm border-2 rounded-md p-4">
  {#if !interval}
    <Button on:click={startMakingRandomMoves}>Start making random moves</Button>
  {:else}
    <Button on:click={stopMakingRandomMoves}>Stop making random moves</Button>
  {/if}

  <span
    >Number of moves found: {numOfMoves}
    <a on:click={getMoves} class="underline cursor-pointer">refresh</a></span
  >
</div>
