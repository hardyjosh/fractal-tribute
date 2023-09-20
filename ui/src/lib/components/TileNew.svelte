<svelte:options accessors />

<script lang="ts">
  import Shape from "$lib/components/Shape.svelte";
  import type { Brush, GameMove, Tile } from "$lib/types";

  export let brush: Brush | null = null;
  export let move: GameMove;
  export let pos: { x: number; y: number };

  export const handleEnter = () => {
    // console.log("should be hovered", brush);
    hovered = true;
  };

  export const handleLeave = () => {
    // console.log("handle hover leave");
    hovered = false;
  };

  let hovered: boolean;

  $: tile = move.changes.find(
    (change) => change.x == pos.y && change.y == pos.x
  );
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="border border-gray-200 relative bg-gray-50 w-full aspect-square">
  <div class="absolute inset-0 flex items-center justify-center z-10">
    {#if brush && hovered && !brush.eyeDropper}
      <Shape color={brush?.color} shapeOption={brush?.graphic_option} />
    {/if}
    <div class="absolute inset-0 flex items-center justify-center z-10">
      {#if tile?.color && tile?.graphic_option !== null}
        <Shape color={tile?.color} shapeOption={tile?.graphic_option} />
      {/if}
    </div>
  </div>
</div>
