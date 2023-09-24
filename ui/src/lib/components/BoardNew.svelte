<script lang="ts">
  import type { Brush } from "$lib/types";
  import { createEventDispatcher } from "svelte";
  import { BOARD_SIZE } from "$lib/helpers";
  import type { BoardWithMetadata, GameMove } from "$lib/types";

  const dispatch = createEventDispatcher();

  export let board: BoardWithMetadata;
  export let brush: Brush | null = null;
  export let allMovesMade: boolean;

  export let move: GameMove;

  let overlay: HTMLDivElement;
  let rect: DOMRectReadOnly;

  let hoveredTile: { x: number; y: number } | null = null;
  $: tilePercent = 100 / BOARD_SIZE;

  $: if (overlay && (brush || allMovesMade)) {
    // console.log("setting cursor", brush);
    if (allMovesMade) {
      // console.log("setting cursor to not allowed");
      overlay.style.cursor = "not-allowed";
    } else if (brush?.eyeDropper) {
      // console.log("setting cursor to eye dropper");
      overlay.style.cursor = "url(/color-picker.cur), auto";
    } else {
      // console.log("setting cursor to cell");
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
      hoveredTile = { x, y };
    }
  };

  const handleLeave = (e) => {
    hoveredTile = null;
  };
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
  <div
    class="absolute inset-0 will-change-auto"
    style="transform: translateZ(0);"
  >
    {@html board.svg}
  </div>

  <svg
    style="transform: translateZ(1);"
    class="absolute inset-0 will-change-auto"
    viewBox={`0 0 ${BOARD_SIZE * 100} ${BOARD_SIZE * 100}`}
    xmlns="http://www.w3.org/2000/svg"
  >
    {#each move.changes as change}
      {#if change.graphic_option < 16}
        <rect
          x={change.y * 100}
          y={change.x * 100}
          width="100"
          height="100"
          fill="white"
        />
        <rect
          x={change.y * 100}
          y={change.x * 100}
          width="100"
          height="100"
          fill={`rgb(${change.color.r}, ${change.color.g}, ${change.color.b})`}
          mask={`url(#m_${(change.graphic_option % 16) + 1})`}
        />
      {:else}
        <rect
          x={change.y * 100}
          y={change.x * 100}
          width="100"
          height="100"
          fill={`rgb(${change.color.r}, ${change.color.g}, ${change.color.b})`}
        />
        <rect
          x={change.y * 100}
          y={change.x * 100}
          width="100"
          height="100"
          fill="white"
          mask={`url(#m_${(change.graphic_option % 16) + 1})`}
        />
      {/if}
    {/each}
    {#if hoveredTile}
      {#if brush?.graphic_option < 16}
        <rect
          x={hoveredTile.x * 100}
          y={hoveredTile.y * 100}
          width="100"
          height="100"
          fill="white"
        />
        <rect
          x={hoveredTile.x * 100}
          y={hoveredTile.y * 100}
          width="100"
          height="100"
          fill={`rgb(${brush?.color.r}, ${brush?.color.g}, ${brush?.color.b})`}
          mask={`url(#m_${(brush?.graphic_option % 16) + 1})`}
        />
      {:else}
        <rect
          x={hoveredTile.x * 100}
          y={hoveredTile.y * 100}
          width="100"
          height="100"
          fill={`rgb(${brush?.color.r}, ${brush?.color.g}, ${brush?.color.b})`}
        />
        <rect
          x={hoveredTile.x * 100}
          y={hoveredTile.y * 100}
          width="100"
          height="100"
          fill="white"
          mask={`url(#m_${(brush?.graphic_option % 16) + 1})`}
        />
      {/if}
    {/if}
  </svg>

  <svg
    style="transform: translateZ(1);"
    class="absolute inset-0 will-change-auto"
    viewBox="0 0 500 500"
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
