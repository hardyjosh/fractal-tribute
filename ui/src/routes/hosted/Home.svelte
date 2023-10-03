<script lang="ts">
  import singlelogo from "$lib/assets/single-logo.svg";
  import { Heading, Button } from "flowbite-svelte";
  import { fade } from "svelte/transition";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata } from "$lib/types";
  import { onDestroy, onMount } from "svelte";
  import GameStats from "$lib/components/GameStats.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";

  let board: BoardWithMetadata;

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
  };

  let pollingInterval;

  onMount(async () => {
    pollingInterval = setInterval(getBoard, 10000);
    getBoard();
  });

  onDestroy(() => {
    clearInterval(pollingInterval);
  });
</script>

<div class="flex flex-row gap-x-8 pt-8 items-center">
  <div in:fade class="flex flex-col gap-y-5 text-center items-center w-1/2">
    <img class="w-32" src={singlelogo} alt="fractal tribute logo" />
    <Heading tag="h2">Welcome to Fractal Tribute!</Heading>
    <Heading tag="h6" class="font-semibold"
      >A peer-to-peer fully distributed, collaborative artistic NFT game.</Heading
    >
    <p class="max-w-md">
      This game began at 10am UTC on Wednesday, October 4th and ends at 3pm UTC
      on Friday October 6th.
    </p>
    <a
      href="https://github.com/holochain-apps/fractal-tribute-kangaroo/releases/tag/v0.0.18"
      class="bg-fractalorange border-2 border-black rounded-2xl py-4 px-8 text-xl font-semibold text-white mt-4"
      >Download app</a
    >
    <div class="w-full mt-12">
      <GameStats />
    </div>
  </div>

  <div
    class="flex flex-row border-black border-2 rounded-lg overflow-hidden relative w-1/2 aspect-square bg-transparent"
  >
    {#if board}
      <div
        in:fade
        class="absolute inset-0 will-change-auto"
        style="transform: translateZ(0);"
      >
        <div>
          <div
            class="absolute top-5 left-5 flex gap-x-2 items-center rounded-full bg-white border-gray-300 border px-4 py-2"
          >
            <div class="rounded-full bg-red-600 animate-pulse w-5 h-5" />
            <span class="font-bold">LIVE</span>
          </div>
          {@html board.svg}
        </div>
      </div>
    {/if}
  </div>
</div>
<div class="w-full flex flex-col gap-y-6">
  <Heading tag="h3" class="mt-14">All snapshots</Heading>
  <span class="text-xl"
    >By minting another player's snapshot you are pushing them up the
    leaderboard!</span
  >
  <AllNfts />
</div>
