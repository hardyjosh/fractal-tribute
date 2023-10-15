<script lang="ts">
  import { fade } from "svelte/transition";
  import type { Brush } from "$lib/types";
  import { createEventDispatcher } from "svelte";
  import { BOARD_SIZE } from "$lib/helpers";
  import type { BoardWithMetadata, GameMove } from "$lib/types";
  import ShapeSvgAlt from "$lib/components/ShapeSvgAlt.svelte";

  const dispatch = createEventDispatcher();

  export let board: BoardWithMetadata;
  export let brush: Brush | null = null;
  export let allMovesMade: boolean;
  export let hideGrid: boolean = false;

  export let move: GameMove;

  let overlay: HTMLDivElement;
  let rect: DOMRectReadOnly;

  let hoveredTile: { x: number; y: number } | null = null;

  $: if (overlay && (brush || allMovesMade)) {
    if (allMovesMade) {
      overlay.style.cursor = "not-allowed";
    } else if (brush.brushTool == "eye-dropper") {
      overlay.style.cursor = "url(/color-picker.cur), auto";
    } else if (brush.brushTool == "eraser") {
      overlay.style.cursor = "url(/eraser.cur), auto";
    } else {
      overlay.style.cursor = "cell";
    }
  }

  const handleClick = (e) => {
    const x = Math.floor((e.offsetX / rect.width) * BOARD_SIZE);
    const y = Math.floor((e.offsetY / rect.height) * BOARD_SIZE);
    dispatch("tileClick", { x, y });
  };

  const handleMove = (e) => {
    const x = Math.floor((e.offsetX / rect.width) * BOARD_SIZE);
    const y = Math.floor((e.offsetY / rect.height) * BOARD_SIZE);

    if (x < 0 || x >= BOARD_SIZE || y < 0 || y >= BOARD_SIZE) {
      hoveredTile = null;
    } else {
      if (brush.brushTool == "none") hoveredTile = { x, y };
    }
  };

  const handleLeave = (e) => {
    hoveredTile = null;
  };

  $: console.log(board);
</script>

<div
  class="flex flex-row border-black border-2 rounded-lg overflow-hidden relative w-full aspect-square bg-transparent"
>
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    on:mousemove={handleMove}
    on:mouseleave={handleLeave}
    on:click={handleClick}
    bind:contentRect={rect}
    bind:this={overlay}
    class="absolute inset-0 z-50"
  />
  {#if board}
    <div
      in:fade
      class="absolute inset-0 will-change-auto"
      style="transform: translateZ(0);"
    >
      {@html board.svg}
      <!-- <img class="absolute inset-0" src={board.png} /> -->
    </div>
  {/if}
  <svg
    style="transform: translateZ(1);"
    class="absolute inset-0 will-change-auto"
    viewBox={`0 0 ${BOARD_SIZE * 100} ${BOARD_SIZE * 100}`}
    xmlns="http://www.w3.org/2000/svg"
  >
    {#each move.changes as change}
      <ShapeSvgAlt
        color={change.color}
        x={change.y * 100}
        y={change.x * 100}
        shapeOption={change.graphic_option}
      />
    {/each}
    {#if hoveredTile}
      <ShapeSvgAlt
        color={brush?.color}
        x={hoveredTile.x * 100}
        y={hoveredTile.y * 100}
        shapeOption={brush?.graphic_option}
      />
    {/if}
  </svg>

  <svg
    style="transform: translateZ(1);"
    class="absolute inset-0 will-change-auto"
    class:invisible={hideGrid}
    viewBox={`0 0 ${BOARD_SIZE * 10} ${BOARD_SIZE * 10}`}
    xmlns="http://www.w3.org/2000/svg"
  >
    <defs>
      <pattern id="grid" width="10" height="10" patternUnits="userSpaceOnUse">
        <path
          d="M 10 0 L 0 0 0 10"
          fill="none"
          stroke="rgb(240, 240, 240)"
          stroke-width="1"
        />
      </pattern>
    </defs>
    <rect width="500" height="500" fill="url(#grid)" />
  </svg>
</div>
