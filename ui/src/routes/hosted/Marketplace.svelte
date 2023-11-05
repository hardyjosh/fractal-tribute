<script lang="ts">
  import { Heading } from "flowbite-svelte";
  import GameStats from "$lib/components/GameStats.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
</script>

<div class="flex flex-row gap-x-8 pt-8 items-center">
  <div class="w-full md:mt-12">
    <GameStats />
  </div>
</div>
<div class="w-full flex flex-col gap-y-6">
  <Heading tag="h3" class="mt-14">Snapshot marketplace</Heading>
  <span class="text-xl"
    >By minting another player's snapshot you are pushing them up the
    leaderboard!</span
  >
  <AllNfts wrap />
</div>
