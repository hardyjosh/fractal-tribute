<script lang="ts">
  import AllocationLeaderboard from "$lib/components/AllocationLeaderboard.svelte";
  import { happ } from "$lib/stores";
  import type { ParticipationProof } from "$lib/types";
  import { onMount } from "svelte";

  let participations: ParticipationProof;

  onMount(async () => {
    participations = await $happ.buildAgentParticipation();
  });
</script>

{#if participations}
  <AllocationLeaderboard {participations} />
{/if}
