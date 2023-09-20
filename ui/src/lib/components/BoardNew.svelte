<script lang="ts">
  import type { Brush } from "$lib/types";
  import { createEventDispatcher } from "svelte";
  import { BOARD_SIZE } from "$lib/helpers";
  import type { BoardWithMetadata, GameMove } from "$lib/types";
  import TileNew from "$lib/components/TileNew.svelte";

  const dispatch = createEventDispatcher();

  export let board: BoardWithMetadata;
  export let brush: Brush | null = null;
  export let allMovesMade: boolean;

  export let move: GameMove;

  let overlay: HTMLDivElement;
  let rect: DOMRectReadOnly;

  let tileArr = Array.from({ length: BOARD_SIZE }, () => Array(BOARD_SIZE));
  let hoveredTile;

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
    const x = Math.floor((e.offsetX / rect.width) * 32);
    const y = Math.floor((e.offsetY / rect.height) * 32);
    // console.log(e);
    dispatch("tileClick", { x, y });
  };

  const handleMove = (e) => {
    // const width = e.client
    const xPos = Math.floor((e.offsetX / rect.width) * 32);
    const yPos = Math.floor((e.offsetY / rect.height) * 32);

    const tile = tileArr[xPos][yPos];
    if (tile !== hoveredTile) {
      tile.handleEnter();
      hoveredTile?.handleLeave();
    }
    hoveredTile = tile;

    // console.log({ xPos, yPos });
  };

  const handleLeave = (e) => {
    hoveredTile?.handleLeave();
  };

  // $: console.log({ rect });
</script>

<div
  class="flex flex-row border-black border-2 rounded-lg overflow-hidden relative w-full aspect-square bg-transparent"
>
  <div
    on:mousemove={handleMove}
    on:mouseleave={handleLeave}
    on:click={handleClick}
    bind:contentRect={rect}
    bind:this={overlay}
    class="absolute inset-0 z-50"
  />

  <div class="w-full flex flex-row">
    {#each tileArr as col, x}
      <div class="w-[3.125%]">
        {#each col as tile, y}
          <TileNew {brush} bind:this={tileArr[x][y]} {move} pos={{ x, y }} />
        {/each}
      </div>
    {/each}
  </div>

  <img class="absolute inset-0" src={board.svg} />
</div>
