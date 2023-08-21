<script lang="ts">
  import { happ } from "$lib/stores";
  import type { Board } from "$lib/types";
  import { Button } from "flowbite-svelte";
  import { twMerge } from "tailwind-merge";

  export let board: Board;
  export let size: string = "w-6 h-6";
  console.log(board);
</script>

{#if board}
  <div class="flex flex-row border border-gray-50">
    {#each board as row}
      <div class="row">
        {#each row as tile}
          <div class={twMerge("w-6 h-6 border border-gray-50 relative", size)}>
            {#if tile?.color}
              <div
                class="absolute inset-0 flex items-center justify-center"
                style={`background-color:rgb(${tile.color.r} ${tile.color.g} ${tile.color.b}`}
              >
                <span class="text-white">
                  {tile.graphic_option}
                </span>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/each}
  </div>
{/if}
