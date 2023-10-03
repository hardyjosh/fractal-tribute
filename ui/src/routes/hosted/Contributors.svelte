<script lang="ts">
  import AllocationLeaderboard from "$lib/components/AllocationLeaderboard.svelte";
  import { happ } from "$lib/stores";
  import type { ParticipationProof } from "$lib/types";
  import { getContext, onMount } from "svelte";
  import type { Writable } from "svelte/store";

  const participations = getContext(
    "participations"
  ) as Writable<ParticipationProof>;
</script>

{#if participations}
  <AllocationLeaderboard participations={$participations} />
{/if}
