<script lang="ts">
  import type { Brush } from "$lib/types";

  export let allMovesMade: boolean;
  export let brush: Brush | null;

  let overlay: HTMLDivElement;
  let rect;

  $: if (overlay && (brush || allMovesMade)) {
    console.log("setting cursor", brush);
    if (allMovesMade) {
      console.log("setting cursor to not allowed");
      overlay.style.cursor = "not-allowed";
    } else if (brush?.eyeDropper) {
      console.log("setting cursor to eye dropper");
      overlay.style.cursor = "url(/color-picker.cur), auto";
    } else {
      console.log("setting cursor to cell");
      overlay.style.cursor = "cell";
    }
  }

  $: console.log(rect);
</script>

<div bind:contentRect={rect} bind:this={overlay} class="absolute inset-0" />
