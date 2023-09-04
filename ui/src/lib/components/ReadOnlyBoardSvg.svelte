<script lang="ts">
  import type { Board } from "$lib/types";
  import { twMerge } from "tailwind-merge";
  import { createEventDispatcher } from "svelte";
  import Shape from "$lib/components/Shape.svelte";
  import ShapeSvg from "$lib/components/ShapeSvg.svelte";

  export let board: Board;
</script>

{#if board}
  <svg
    class="border-2 border-black rounded-lg"
    viewBox="0 0 3200 3200"
    xmlns="http://www.w3.org/2000/svg"
  >
    {#each board as col, x}
      {#each col as tile, y}
        <!-- Rectangle for the grid -->
        <rect
          x={x * 100 + 1}
          y={y * 100 + 1}
          width="98"
          height="98"
          fill="rgb(249, 250, 251)"
        />
        <ShapeSvg
          shapeOption={tile?.graphic_option}
          color={tile?.color || { r: 255, g: 255, b: 255 }}
          position={{ x: x * 100, y: y * 100 }}
        />
        <!-- <rect
          x={x * 100}
          y={y * 100}
          width="100"
          height="100"
          stroke="rgb(243, 244, 246)"
          stroke-width="6"
          fill="none"
        /> -->
      {/each}
    {/each}
  </svg>
{/if}

<style>
  .tile:hover {
    background-color: rgb(var(--r), var(--g), var(--b));
  }
</style>
