<script lang="ts">
  import type { Writable } from "svelte/store";
  import { Heading } from "flowbite-svelte";
  import { formatUnits } from "viem";
  import { happ, paymentTokenAddress, language } from "$lib/stores";
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

  const getStats = async (nfts) => {
    const totalCollected = nfts.reduce(
      (acc, nft) => acc + Number(nft.supply),
      0
    );
    const totalSnapshots = nfts.length;
    const poolSize = (BigInt(totalCollected) - BigInt(totalSnapshots)) * price;
    const token = await fetchToken({
      address: $paymentTokenAddress,
      chainId: $happ.dnaProperties.chainId,
    });
    poolsizeFormatted = formatUnits(poolSize, token.decimals);
    ready = true;
  };

  $: getStats($nfts);
</script>

<div class="flex flex-col gap-y-4 grow">
  <div
    class="flex flex-col md:flex-row justify-between gap-4 rounded-xl bg-gray-50 p-8"
  >
    <ParticipationStat
      {ready}
      name={$language == "en"
        ? "Total pool size"
        : "Toplam Fon Havuzu Büyüklüğü"}
      value={`${poolsizeFormatted} MATIC`}
    />
    <ParticipationStat
      ready={!!$participations?.agent_participations.length}
      name={$language == "en" ? "Number of creators" : "Yaratıcı Sayısı"}
      value={$participations?.agent_participations.length.toString()}
    />
    <ParticipationStat
      ready={!!$participations?.total_pixels_changed}
      name={$language == "en"
        ? "Total pixels changed"
        : "Değiştirilen Toplam Piksel"}
      value={$participations?.total_pixels_changed.toString()}
    />
    {#if $countdown.timeRemaining}
      <ParticipationStat
        {ready}
        name={$language == "en" ? "Game time remaining" : "Kalan Oyun Süresi"}
        value={formatCountdown($countdown)}
      />
    {:else if $snapshotEndCountdown.timeRemaining}
      <ParticipationStat
        {ready}
        name={$language == "en" ? "Mint time remaining" : "Kalan Mint Süresi"}
        value={formatCountdown($snapshotEndCountdown)}
      />
    {:else}
      <ParticipationStat
        {ready}
        name={$language == "en" ? "Time remaining" : "Kalan Süre"}
        value={$language == "en" ? "Game ended" : "Oyun Bitti"}
      />
    {/if}
  </div>
</div>
