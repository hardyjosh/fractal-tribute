<script lang="ts">
  import type { Board } from "$lib/types";
  import { twMerge } from "tailwind-merge";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let board: Board;
  export let size: string = "w-7 h-7";
  export let brush: {
    color: { r: number; g: number; b: number };
    graphic_option: number;
  } | null = null;
  export let notAllowed: boolean = false;

  const handleTileClick = (x: number, y: number) => {
    dispatch("tileClick", { x, y });
  };
</script>

{#if board}
  <div class="flex flex-row border-black border-2 rounded-lg overflow-hidden">
    {#each board as row, x}
      <div class="row">
        {#each row as tile, y}
          <div
            role="button"
            tabindex="0"
            class={twMerge(
              "w-6 h-6 border border-gray-100 relative hover:border hover:border-blue-700 cursor-cell bg-gray-50",
              size,
              tile.changed && "border-blue-700 border-4 border-double",
              notAllowed && "cursor-not-allowed"
            )}
            on:mouseenter={(e) => {
              e.currentTarget.style.setProperty(
                "background-color",
                `rgb(${brush?.color.r} ${brush?.color.g} ${brush?.color.b})`
              );
            }}
            on:mouseleave={(e) => {
              e.currentTarget.style.setProperty("background-color", "");
            }}
            on:click={() => handleTileClick(x, y)}
            on:keydown={(event) => {
              if (event.key === "Enter" || event.key === " ") {
                handleTileClick(x, y);
              }
            }}
          >
            {#if tile?.color}
              <div
                class="absolute inset-0 flex items-center justify-center"
                style={`background-color:rgb(${tile.color.r} ${tile.color.g} ${tile.color.b}`}
              >
                <!-- <span class="text-white">
                  {tile.graphic_option}
                </span> -->
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/each}
  </div>
{/if}

<style>
  .tile:hover {
    background-color: rgb(var(--r), var(--g), var(--b));
  }
</style>
