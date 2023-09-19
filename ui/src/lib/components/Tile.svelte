<script lang="ts">
  import Shape from "$lib/components/Shape.svelte";
  import type { Brush, Tile } from "$lib/types";
  import { twMerge } from "tailwind-merge";

  export let tile: Tile;
  export let brush: Brush | null = null;

  let hovered: boolean;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class={twMerge(
    "border border-gray-100 relative bg-gray-50 hover:border hover:border-blue-700",
    "w-full aspect-square",
    tile.changed && "border-blue-700 border-4 border-double"
  )}
  on:mouseenter={(e) => {
    if (brush?.eyeDropper) return;
    hovered = true;
  }}
  on:mouseleave={(e) => {
    if (brush?.eyeDropper) return;
    hovered = false;
  }}
  on:click
>
  <div class="absolute inset-0 flex items-center justify-center z-10">
    {#if brush && hovered}
      <Shape color={brush?.color} shapeOption={brush?.graphic_option} />
    {/if}
  </div>
  <div class="absolute inset-0 flex items-center justify-center z-0">
    {#if tile?.color && tile?.graphic_option !== null}
      <Shape color={tile?.color} shapeOption={tile?.graphic_option} />
    {/if}
  </div>
</div>
