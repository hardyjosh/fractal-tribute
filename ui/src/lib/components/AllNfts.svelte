<script lang="ts">
  import { fade } from "svelte/transition";
  import { formatAddress } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { Button, Modal } from "flowbite-svelte";
  import type { BoardWithMetadataAndId } from "$lib/types";
  import { encodeHashToBase64 } from "@holochain/client";
  import no_snapshots from "$lib/assets/no_snapshots.svg";
  import MintMove from "$lib/components/MintMove.svelte";
  import { hexToBigInt, type Hex, bytesToHex } from "viem";
  import { nfts } from "$lib/stores/nfts";
  import Identicon from "$lib/components/Identicon.svelte";
  import { getContext } from "svelte";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";
  import IntersectionObserver from "svelte-intersection-observer";
  import NftCard from "$lib/components/NftCard.svelte";

  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

  export let wrap: boolean = false;
  let wrappers: HTMLElement[] = [];

  let boards: BoardWithMetadataAndId[] = [];
  let boardsWithSupply: (BoardWithMetadataAndId & { supply: number })[];

  const prepareHappNfts = async () => {
    boards = await $happ.getBoardsFromTokenIds($nfts.map((nft) => nft.id));
    boardsWithSupply = boards.map((board) => {
      const nft = $nfts.find((nft) => bytesToHex(nft.id) == board.id);
      return { ...board, supply: nft.supply };
    });
    boardsWithSupply.sort((a, b) => b.supply - a.supply);
  };

  $: if ($nfts) prepareHappNfts();

  let mintMoveModal = false;
  let tokenId: bigint;

  const mintMove = (id: Hex) => {
    mintMoveModal = true;
    tokenId = hexToBigInt(id);
  };
</script>

{#if boardsWithSupply}
  {#if boardsWithSupply?.length == 0}
    <div
      class="w-full rounded-lg border-2 border-black flex flex-col gap-2 items-center justify-center h-80"
    >
      <img src={no_snapshots} alt="no snapshots" />
      <p class="text-2xl font-semibold">No snapshots yet</p>
      <p>
        Snapshots from you and other players will appear here once minted
        onchain.
      </p>
    </div>
  {:else}
    <div
      class:flex-wrap={wrap}
      class="flex overflow-scroll gap-4 will-change-scroll"
    >
      {#each boardsWithSupply as board (board.id)}
        <IntersectionObserver
          once
          element={wrappers[board.id]}
          let:intersecting
        >
          <div
            bind:this={wrappers[board.id]}
            class="relative flex flex-col gap-y-2 flex-none snap-start basis-1/5-gap-4"
          >
            {#if intersecting}
              <NftCard
                {board}
                mintDisabled={$snapshotEndCountdown?.timeRemaining == 0}
              />
            {/if}
          </div>
        </IntersectionObserver>
      {/each}
    </div>
  {/if}
{/if}

<Modal permanent bind:open={mintMoveModal}>
  <MintMove bind:open={mintMoveModal} {tokenId} />
</Modal>
