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

  const participations = getContext(
    "participations"
  ) as Writable<ParticipationProof>;
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
      name="Number of players"
      value={$participations?.agent_participations.length.toString()}
    />
    <ParticipationStat
      {ready}
      name="Total pixels changed"
      value={$participations?.total_pixels_changed.toString()}
    />
  </div>
</div>
