<script lang="ts">
  import { happ } from "$lib/stores";
  import type { ParticipationProof } from "$lib/types";
  import { onMount } from "svelte";
  import Participation from "$lib/components/Participation.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import { Heading } from "flowbite-svelte";
  import AllocationLeaderboard from "$lib/components/AllocationLeaderboard.svelte";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

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
      <Heading tag="h3" class="w-auto"
        ><En>Leaderboard</En><Tr>Liderlik Panosu</Tr></Heading
      >
      <button
        class="tab-button"
        class:tab-button-active={tabOption === "most-minted"}
        on:click={() => {
          tabOption = "most-minted";
          console.log("clicked", tabOption);
        }}><En>Most Minted</En><Tr>En fazla basılanlar (mintlenen)</Tr></button
      >
      <button
        class="tab-button"
        class:tab-button-active={tabOption === "highest-allocation"}
        on:click={() => (tabOption = "highest-allocation")}
        disabled={!participations}
        ><En>Highest Allocation</En><Tr>En fazla tahsis edilenler</Tr></button
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
