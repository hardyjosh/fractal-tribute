<script lang="ts">
  import { BOARD_SIZE } from "$lib/helpers";
  import type { GameMove } from "$lib/types";

  export let move: GameMove;
</script>

<svg
  style="transform: translateZ(1);"
  class="absolute inset-0"
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
</svg>
