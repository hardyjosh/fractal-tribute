<script lang="ts">
  import { nfts } from "$lib/stores/nfts";
  import { happ, language, paymentTokenAddress } from "$lib/stores";
  import type { AgentParticipation, ParticipationProof } from "$lib/types";
  import { Heading } from "flowbite-svelte";
  import { bytesToHex, formatUnits } from "viem";
  import { fetchToken, type FetchTokenResult } from "@wagmi/core";
  import ParticipationStat from "$lib/components/ParticipationStat.svelte";
  import { getContext } from "svelte";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";
  import Claim from "$lib/components/Claim.svelte";
  import { price } from "$lib/constants";

  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

  export let participations: ParticipationProof;
  let myParticipation: AgentParticipation;
  let poolSize: bigint,
    token: FetchTokenResult,
    poolsizeFormatted: string,
    ready: boolean;

  const getStats = async () => {
    const myPubKey = await $happ.myPubKey();
    myParticipation = participations.agent_participations.find(
      (p) => bytesToHex(p.agent) == bytesToHex(myPubKey)
    );

    const totalCollected = $nfts.reduce(
      (acc, nft) => acc + Number(nft.supply),
      0
    );
    const totalSnapshots = $nfts.length;
    poolSize = (BigInt(totalCollected) - BigInt(totalSnapshots)) * price;

    token = await fetchToken({
      address: $paymentTokenAddress,
      chainId: $happ.dnaProperties.chainId,
    });

    poolsizeFormatted = formatUnits(poolSize, token.decimals);
    ready = true;
  };

  $: if (participations) getStats();
</script>

<div class="flex flex-col gap-y-4">
  <Heading tag="h3" class="mt-14">My Stats</Heading>
  <div class="flex justify-between gap-4 rounded-xl bg-gray-50 p-8">
    <ParticipationStat
      {ready}
      name={$language == "en"
        ? "Your pixels changed"
        : "Değiştirilen piksellerim"}
      value={`${
        myParticipation?.pixels_changed?.toString() || 0
      } / ${participations?.total_pixels_changed?.toString()}`}
    />
    <ParticipationStat
      {ready}
      name={$language == "en" ? "Percentage allocation" : "Tahsis yüzdesi"}
      value="{(
        (myParticipation?.percentage_of_total_pixels_changed || 0) * 100
      )?.toPrecision(4)}%"
    />
    <ParticipationStat
      {ready}
      name={$language == "en" ? "Total pool size" : "Havuzdaki toplam miktar"}
      value={`${poolsizeFormatted} MATIC`}
    />
    <div class="flex flex-col gap-y-2">
      <ParticipationStat
        {ready}
        name={$language == "en"
          ? "Your current allocation"
          : "Sana tahsis edilen güncel miktar"}
        value={`${(
          myParticipation?.percentage_of_total_pixels_changed *
            Number(poolsizeFormatted) || 0
        ).toPrecision(4)} MATIC`}
      />
      {#if !$snapshotEndCountdown?.timeRemaining && participations && myParticipation?.percentage_of_total_pixels_changed > 0}
        <Claim {participations} />
      {/if}
    </div>
  </div>
</div>
