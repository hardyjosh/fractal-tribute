<script lang="ts">
  import type { CountdownObject } from "$lib/stores/countdown";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata } from "$lib/types";
  import { onMount } from "svelte";
  import Participation from "$lib/components/Participation.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import { Heading } from "flowbite-svelte";
  import Countdown from "$lib/components/Countdown.svelte";
  import { countdownContext } from "$lib/contexts";
  import { getContext } from "svelte";
  import type { Readable } from "svelte/store";

  let board: BoardWithMetadata;

  onMount(async () => {
    board = await $happ.getLatestBoard();
  });

  const countdown = getContext(countdownContext) as Readable<CountdownObject>;
</script>

<div class="flex flex-col gap-y-8">
  <div class="grid grid-cols-5 gap-x-4">
    <div
      class="col-span-3 aspect-square relative border-black border-2 rounded-md"
    >
      <!-- <BoardComp readOnly {board} /> -->
      <!-- <ReadOnlyBoardSvg {board} /> -->
      <!-- <img
        alt="game board"
        class="border-black border-2 rounded-md"
        src={board?.svg}
      /> -->
      {#if board}
      {@html board?.svg}
      {/if}
    </div>
    <div
      class="col-span-2 bg-primary-25 border-2 border-black rounded-lg flex flex-col items-center pt-12 gap-y-8"
    >
      <Heading tag="h4" class="font-pixel text-center">Game ends in</Heading>
      <Countdown countdown={$countdown} />
      <Participation />
    </div>
  </div>
  <AllNfts showWinnerPanel heading="Current winners 🎉" />
  <AllNfts heading="Snapshots" />
</div>
