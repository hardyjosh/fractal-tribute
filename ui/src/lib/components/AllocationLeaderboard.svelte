<script lang="ts">
  import { fade } from "svelte/transition";
  import type { ParticipationProof } from "$lib/types";
  import { onMount } from "svelte";
  import Identicon from "./Identicon.svelte";
  import { happ } from "$lib/stores";
  import { formatAddress } from "$lib/helpers";
  import { encodeHashToBase64 } from "@holochain/client";

  export let participations: ParticipationProof;

  onMount(() => {
    // sort the participations by percentage of total pixels changed
    participations.agent_participations.sort((a, b) => {
      return (
        b.percentage_of_total_pixels_changed -
        a.percentage_of_total_pixels_changed
      );
    });
  });
</script>

<div in:fade|global={{ duration: 200 }}>
  {#each participations.agent_participations as agent}
    <div
      class="flex items-center justify-between py-2 border-b border-gray-300"
    >
      <div class="flex items-center gap-x-4 text-lg">
        <Identicon agentHash={agent.agent} />
        {#await $happ.getProfile(agent.agent) then profile}
          {profile.name}
        {:catch error}
          {formatAddress(encodeHashToBase64(agent.agent))}
        {/await}
      </div>
      <div class="flex items-center">
        <div class="border-r border-gray-300 pr-2 mr-2">
          {(agent.percentage_of_total_pixels_changed * 100).toPrecision(4)}%
        </div>
        <div>{agent.pixels_changed} pixels</div>
      </div>
    </div>
  {/each}
</div>
