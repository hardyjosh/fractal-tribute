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
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

  export let wrap: boolean = false;
  let wrappers: HTMLElement[] = [];

  let boards: BoardWithMetadataAndId[] = [];
  let boardsWithSupply: (BoardWithMetadataAndId & { supply: number })[] = [];

  const prepareHappNfts = async (nfts) => {
    // Filter out any NFTs that are already represented in boardsWithSupply
    nfts = nfts.filter(
      (nft) => !boardsWithSupply.find((board) => board.id === nft.id)
    );
    const tokenIds = nfts.map((nft) => nft.id);

    // Batch out the async requests for getBoardsFromTokenIds to 5 each
    const boardPromises = tokenIds.reduce((promiseArray, _, i) => {
      if (i % 5 === 0) {
        const batch = tokenIds.slice(i, i + 5);
        promiseArray.push($happ.getBoardsFromTokenIds(batch));
      }
      return promiseArray;
    }, []);

    // Process each promise as it resolves
    boardPromises.forEach((boardPromise) => {
      boardPromise.then((boards) => {
        const _boardsWithSupply = boards.map((board) => {
          const nft = nfts.find((nft) => bytesToHex(nft.id) === board.id);
          return { ...board, supply: nft.supply };
        });

        // push the new boards to the array and then sort by supply
        boardsWithSupply.push(..._boardsWithSupply);
        boardsWithSupply.sort((a, b) => b.supply - a.supply);
        boardsWithSupply = [...boardsWithSupply];
      });
    });
  };

  $: if ($nfts) prepareHappNfts($nfts);

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
      class="w-full rounded-lg border-2 border-black flex flex-col gap-2 items-center justify-center h-80 text-center"
    >
      <img src={no_snapshots} alt="no snapshots" />
      <p class="text-2xl font-semibold">
        <En>No snapshots yet</En><Tr>Henüz hiçbir görüntü yok</Tr>
      </p>
      <p>
        <En>
          Snapshots from you and other players will appear here once minted
          onchain.</En
        ><Tr
          >Senin ve diğer oyuncuların aldığı ekran görüntüleri, zincir üzerinde
          basıldığında (mintlendiğinde) burada görünecek</Tr
        >
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
            class="relative flex flex-col gap-y-2 flex-none snap-start md:basis-1/5-gap-4 w-full"
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
