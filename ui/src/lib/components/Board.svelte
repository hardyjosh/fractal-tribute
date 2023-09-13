<script lang="ts">
  import { fade } from "svelte/transition";
  import type { Board } from "$lib/types";
  import { twMerge } from "tailwind-merge";
  import { createEventDispatcher } from "svelte";
  import Shape from "$lib/components/Shape.svelte";
  import { Spinner } from "flowbite-svelte";

  const dispatch = createEventDispatcher();

  export let board: Board;
  export let readOnly: boolean = false;
  export let brush: {
    color: { r: number; g: number; b: number };
    graphic_option: number;
  } | null = null;
  export let eyeDropper;
  export let notAllowed: boolean = false;

  let hoveredTile: { x: number; y: number } | null = null;

  const handleTileClick = (x: number, y: number) => {
    if (readOnly) return;
    dispatch("tileClick", { x, y });
  };
</script>

<div
  class="flex flex-row border-black border-2 rounded-lg overflow-hidden relative w-full aspect-square items-center justify-center"
>
  {#if board}
    {#each board as col, x}
      <div in:fade class="w-[3.125%]">
        {#each col as tile, y}
          <div
            role="button"
            tabindex="0"
            class={twMerge(
              "border border-gray-100 relative bg-gray-50 cursor-default",
              !readOnly && "hover:border hover:border-blue-700 cursor-cell",
              "w-full aspect-square",
              tile.changed && "border-blue-700 border-4 border-double",
              notAllowed && "cursor-not-allowed",
              eyeDropper && "cursor-[url(/color-picker.cur),_auto]"
            )}
            on:mouseenter={(e) => {
              if (readOnly || eyeDropper) return;
              hoveredTile = { x, y };
            }}
            on:mouseleave={(e) => {
              if (readOnly || eyeDropper) return;
              hoveredTile = null;
            }}
            on:click={() => handleTileClick(x, y)}
            on:keydown={(event) => {
              if (event.key === "Enter" || event.key === " ") {
                handleTileClick(x, y);
              }
            }}
          >
            <div class="absolute inset-0 flex items-center justify-center z-10">
              {#if brush?.color && brush?.graphic_option !== null && !eyeDropper && hoveredTile?.x === x && hoveredTile?.y === y}
                <Shape
                  color={brush?.color}
                  shapeOption={brush?.graphic_option}
                />
              {/if}
            </div>
            <div class="absolute inset-0 flex items-center justify-center z-0">
              {#if tile?.color && tile?.graphic_option !== null}
                <Shape color={tile?.color} shapeOption={tile?.graphic_option} />
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/each}
  {:else}
    <Spinner />
  {/if}
</div>

<style>
  .tile:hover {
    background-color: rgb(var(--r), var(--g), var(--b));
  }
</style>
