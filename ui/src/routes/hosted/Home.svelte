<script lang="ts">
  import { fade } from "svelte/transition";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata } from "$lib/types";
  import { onDestroy, onMount } from "svelte";

  let board: BoardWithMetadata;

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
    console.log(board);
  };

  let pollingInterval;

  onMount(async () => {
    pollingInterval = setInterval(getBoard, 10000);
    getBoard();
  });

  onDestroy(() => {
    clearInterval(pollingInterval);
  });
</script>

<div
  class="flex flex-row border-black border-2 rounded-lg overflow-hidden relative w-1/2 mx-auto aspect-square bg-transparent"
>
  {#if board}
    <div
      in:fade
      class="absolute inset-0 will-change-auto"
      style="transform: translateZ(0);"
    >
      {@html board.svg}
    </div>
  {/if}
</div>
