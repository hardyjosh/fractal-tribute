<script lang="ts">
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata, ParticipationProof } from "$lib/types";
  import { getContext, onMount } from "svelte";
  import Participation from "$lib/components/Participation.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import { Heading } from "flowbite-svelte";
  import AllocationLeaderboard from "$lib/components/AllocationLeaderboard.svelte";
  import Claim from "$lib/components/Claim.svelte";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";

  let tabOption: "most-minted" | "highest-allocation" = "most-minted";
  let participations: ParticipationProof;

  onMount(async () => {
    participations = await $happ.buildAgentParticipation();
  });
</script>

<div class="flex flex-col gap-y-24">
  <Participation {participations} />

  <div class="flex flex-col gap-y-4">
    <div class="flex gap-x-4 items-center">
      <Heading tag="h3" class="w-auto">Leaderboard</Heading>
      <button
        class="tab-button"
        class:tab-button-active={tabOption === "most-minted"}
        on:click={() => {
          tabOption = "most-minted";
          console.log("clicked", tabOption);
        }}>Most Minted</button
      >
      <button
        class="tab-button"
        class:tab-button-active={tabOption === "highest-allocation"}
        on:click={() => (tabOption = "highest-allocation")}
        disabled={!participations}>Highest Allocation</button
      >
    </div>
    {#if tabOption == "most-minted"}
      <AllNfts wrap />
    {:else if (tabOption = "highest-allocation")}
      <AllocationLeaderboard {participations} />
    {/if}
  </div>
</div>

<style lang="postcss">
  .tab-button {
    @apply px-4 py-3 rounded-lg leading-none transition-all duration-100;
  }

  .tab-button-active {
    @apply bg-fractalorange text-white;
  }
</style>
