<script lang="ts">
  import { onMount } from "svelte";
  import { happ, paymentTokenAddress } from "$lib/stores";
  import type {
    AgentParticipation,
    Board,
    ParticipationProof,
  } from "$lib/types";
  import { Button, Heading } from "flowbite-svelte";
  import { bytesToHex, formatUnits, type Address, type Hex } from "viem";
  import addresses from "$lib/addresses.json";
  import { fetchToken, readContract, type FetchTokenResult } from "@wagmi/core";
  import { setRoute } from "$lib/stores/routes";
  import { IERC20 } from "$lib/abi/IERC20";
  import ParticipationStat from "$lib/components/ParticipationStat.svelte";

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
    poolSize = await readContract({
      address: $happ.dnaProperties.paymentTokenAddress,
      abi: IERC20,
      functionName: "balanceOf",
      args: [addresses.instance as Hex],
    });
    token = await fetchToken({ address: $paymentTokenAddress });
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
      name="Your pixels changed"
      value={`${
        myParticipation?.pixels_changed?.toString() || 0
      } / ${participations?.total_pixels_changed?.toString()}`}
    />
    <ParticipationStat
      {ready}
      name="Percentage allocation"
      value="{(
        (myParticipation?.percentage_of_total_pixels_changed || 0) * 100
      )?.toPrecision(4)}%"
    />
    <ParticipationStat
      {ready}
      name="Total pool size"
      value={`${poolsizeFormatted} MATIC`}
    />
    <ParticipationStat
      {ready}
      name="Your current allocation"
      value={`${(
        myParticipation?.percentage_of_total_pixels_changed *
          Number(poolsizeFormatted) || 0
      ).toPrecision(2)} MATIC`}
    />
  </div>
</div>
