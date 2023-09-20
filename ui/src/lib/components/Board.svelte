<script lang="ts">
  import type { Board, Brush } from "$lib/types";
  import { twMerge } from "tailwind-merge";
  import { createEventDispatcher } from "svelte";
  import { Spinner } from "flowbite-svelte";
  import Tile from "$lib/components/Tile.svelte";
  import { afterUpdate } from "svelte";

  const dispatch = createEventDispatcher();

  export let board: Board;
  export let brush: Brush | null = null;

  let wrapper: HTMLDivElement;

  const handleTileClick = (x: number, y: number) => {
    dispatch("tileClick", { x, y });
  };
</script>

<div
  bind:this={wrapper}
  class={"flex flex-row border-black border-2 rounded-lg overflow-hidden relative w-full aspect-square items-center justify-center will-change-auto"}
>
  {#if board}
    {#each board as col, x}
      <div class="w-[3.125%]">
        {#each col as tile, y}
          <Tile
            {brush}
            {tile}
            on:click={() => {
              handleTileClick(x, y);
            }}
          />
        {/each}
      </div>
    {/each}
  {:else}
    <Spinner />
  {/if}
</div>
