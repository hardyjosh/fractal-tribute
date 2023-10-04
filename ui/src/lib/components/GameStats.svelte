<script lang="ts">
  import type { Writable } from "svelte/store";
  import { Heading } from "flowbite-svelte";
  import { formatUnits } from "viem";
  import { paymentTokenAddress } from "$lib/stores";
  import { fetchToken } from "@wagmi/core";
  import { nfts } from "$lib/stores/nfts";
  import { price } from "$lib/constants";
  import ParticipationStat from "$lib/components/ParticipationStat.svelte";
  import { getContext, onMount } from "svelte";
  import type { ParticipationProof } from "$lib/types";
  import { formatCountdown } from "$lib/stores/countdown";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";

  const participations = getContext(
    "participations"
  ) as Writable<ParticipationProof>;
  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;
  let poolsizeFormatted: string;
  let ready: boolean;

  const getStats = async () => {
    const totalCollected = $nfts.reduce(
      (acc, nft) => acc + Number(nft.supply),
      0
    );
    const totalSnapshots = $nfts.length;
    const poolSize = (BigInt(totalCollected) - BigInt(totalSnapshots)) * price;
    const token = await fetchToken({ address: $paymentTokenAddress });
    poolsizeFormatted = formatUnits(poolSize, token.decimals);
    ready = true;
  };

  onMount(() => {
    getStats();
  });
</script>

<div class="flex flex-col gap-y-4 grow">
  <div class="flex justify-between gap-4 rounded-xl bg-gray-50 p-8">
    <ParticipationStat
      {ready}
      name="Total pool size"
      value={`${poolsizeFormatted} MATIC`}
    />
    <ParticipationStat
      {ready}
      name="Number of creators"
      value={$participations?.agent_participations.length.toString()}
    />
    <ParticipationStat
      {ready}
      name="Total pixels changed"
      value={$participations?.total_pixels_changed.toString()}
    />
    {#if $countdown.timeRemaining}
      <ParticipationStat
        {ready}
        name="Game time remaining"
        value={formatCountdown($countdown)}
      />
    {:else if $snapshotEndCountdown.timeRemaining}
      <ParticipationStat
        {ready}
        name="Mint time remaining"
        value={formatCountdown($snapshotEndCountdown)}
      />
    {:else}
      <ParticipationStat {ready} name="Time remaining" value="Game ended" />
    {/if}
  </div>
</div>
