<script lang="ts">
  import { paymentTokenAddress } from "$lib/stores";
  import { fetchToken } from "@wagmi/core";
  import { getAddress, bytesToHex, formatUnits } from "viem";
  import { account } from "svelte-wagmi-stores";
  import { web3modal } from "$lib/stores";
  import { formatCountdown } from "$lib/stores/countdown";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";
  import type { ParticipationProof } from "$lib/types";
  import { getContext, onMount } from "svelte";
  import type { Writable } from "svelte/store";
  import { Button } from "flowbite-svelte";
  import { nfts } from "$lib/stores/nfts";
  import { price } from "$lib/constants";
  import Claim from "$lib/components/Claim.svelte";

  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

  const participations = getContext(
    "participations"
  ) as Writable<ParticipationProof>;

  let myParticipations: ParticipationProof["agent_participations"] = [];

  $: if ($account.address) {
    myParticipations = $participations.agent_participations.filter(
      (p) => getAddress(bytesToHex(p.evm_key)) == getAddress($account.address)
    );
  }

  let poolsizeFormatted: string, poolSize: bigint;
  let ready: boolean;

  const getStats = async () => {
    const totalCollected = $nfts.reduce(
      (acc, nft) => acc + Number(nft.supply),
      0
    );
    const totalSnapshots = $nfts.length;
    poolSize = (BigInt(totalCollected) - BigInt(totalSnapshots)) * price;
    const token = await fetchToken({ address: $paymentTokenAddress });
    poolsizeFormatted = formatUnits(poolSize, token.decimals);
    ready = true;
  };

  onMount(() => {
    getStats();
  });
</script>

<div
  class="max-w-xl w-full rounded-2xl bg-white p-8 border border-gray-100 flex flex-col items-center justify-center mx-auto mt-20 shadow-md"
>
  {#if false && $snapshotEndCountdown?.timeRemaining}
    <span>Claims open in {formatCountdown($snapshotEndCountdown)}</span>
  {:else if !$account?.isConnected}
    <div class="flex flex-col gap-y-4">
      <span class="text-xl font-semibold">Connect your wallet to claim</span>
      <p>Your wallet must be connected to the Polygon network to claim.</p>
      <Button
        class="bg-fractalorange border-2 border-black"
        on:click={$web3modal.openModal()}>Connect wallet</Button
      >
    </div>
  {:else if myParticipations.length}
    <div class="flex flex-col gap-y-8">
      <span class="font-semibold text-xl"
        >There are {myParticipations.length} players/s bound to the connected wallet.</span
      >
      <div class="flex flex-col gap-y-2 w-full">
        {#each myParticipations as participation}
          <div
            class="bg-gray-50 rounded-lg w-full p-8 flex gap-x-2 justify-between items-center"
          >
            <div class="flex flex-col">
              <span>Contribution</span>
              <span class="font-semibold"
                >{participation.pixels_changed.toString()}/{$participations.total_pixels_changed}
                pixels changed ({(
                  participation.percentage_of_total_pixels_changed * 100
                ).toPrecision(6)}%)</span
              >
              <span class="mt-3">Share of pool</span>
              <span class="font-semibold"
                >{(
                  participation.percentage_of_total_pixels_changed *
                  Number(poolsizeFormatted)
                ).toPrecision(6)} WMATIC</span
              >
            </div>
            <Claim
              evmKey={bytesToHex(participation.evm_key)}
              participations={$participations}
            />
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <span
      >There are no players bound to the connected wallet. Are you connected
      with the correct account?</span
    >
  {/if}
</div>
