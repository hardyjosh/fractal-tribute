<script lang="ts">
  import BoardComp from "$lib/components/Board.svelte";
  import { happ } from "$lib/stores";
  import type { Board } from "$lib/types";
  import { onMount } from "svelte";
  import Participation from "$lib/components/Participation.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";

  let board: Board;

  onMount(async () => {
    board = await $happ.getLatestBoard();
  });
</script>

<div class="flex flex-col gap-y-4">
  <div class="grid grid-cols-5 gap-x-4">
    <div class="col-span-3">
      <BoardComp readOnly {board} />
    </div>
    <div class="col-span-2 bg-primary-50 border-2 border-black rounded-lg">
      <Participation />
    </div>
  </div>
  <AllNfts />
</div>
