<script lang="ts">
  import { formatAddress } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { Button, Heading, Modal } from "flowbite-svelte";
  import type { BoardWithMetadataAndId } from "$lib/types";
  import { encodeHashToBase64 } from "@holochain/client";
  import no_snapshots from "$lib/assets/no_snapshots.svg";
  import MintMove from "$lib/components/MintMove.svelte";
  import { hexToBigInt, type Hex, bytesToHex } from "viem";
  import { nfts } from "$lib/stores/nfts";

  export let heading = "Latest snapshots";
  export let showWinnerPanel = false;

  let boards: BoardWithMetadataAndId[] = [];
  let boardsWithSupply: (BoardWithMetadataAndId & { supply: number })[] = [];

  const prepareHappNfts = async () => {
    boards = await $happ.getBoardsFromTokenIds($nfts.map((nft) => nft.id));
    boardsWithSupply = boards.map((board) => {
      const nft = $nfts.find((nft) => bytesToHex(nft.id) == board.id);
      return { ...board, supply: nft.supply };
    });
    if (showWinnerPanel) {
      boardsWithSupply.sort((a, b) => b.supply - a.supply);
    }
  };

  $: if ($nfts) prepareHappNfts();

  let mintMoveModal = false;
  let tokenId: bigint;

  const mintMove = (id: Hex) => {
    mintMoveModal = true;
    tokenId = hexToBigInt(id);
  };
</script>

<div class="flex flex-col gap-y-2">
  <Heading tag="h4" class="font-pixel">{heading}</Heading>
  <p class="text-lg">Vote for your favourite snapshots by minting them</p>
</div>
<div class="flex overflow-scroll gap-4">
  {#if !boardsWithSupply || boardsWithSupply?.length == 0}
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
    {#if showWinnerPanel}
      <div
        class="bg-fractalorange border-black border-2 rounded-lg w-60 flex flex-col items-center justify-center p-4 gap-y-2"
      >
        <div
          class="h-20 w-20 rounded-full bg-white bg-opacity-20 flex flex-col items-center justify-center mb-6"
        >
          <span class="text-3xl">🎉</span>
        </div>
        <div class="text-xl text-white font-medium text-center">
          Current Winners
        </div>
        <p class="text-white text-center">
          The most collected snapshots for this game
        </p>
      </div>
    {/if}
    {#each boardsWithSupply as board, i}
      <div
        class="relative flex flex-col gap-y-2 basis-1/5-gap-4 flex-none snap-start"
      >
        <div class="aspect-square border-2 border-black rounded-lg">
          {@html board.boardWithMetadata.svg}
        </div>
        <!-- <img
          class="border-2 border-black rounded-lg"
          src={board.boardWithMetadata.svg}
          alt="game board"
        /> -->
        <div
          class="rounded-md border-black border-2 flex gap-x-2 p-2 justify-between items-center w-full"
        >
          <div class="flex flex-col">
            <span class="text-gray-500 font-light"
              >{formatAddress(
                encodeHashToBase64(board.boardWithMetadata.creator)
              )}</span
            >
            <span>{board.supply} minted</span>
          </div>
          <Button
            on:click={() => {
              mintMove(board.id);
            }}
            class="!bg-primary-500 border-black border-2"
            size="sm">Mint</Button
          >
        </div>
        <!-- <MintMove tokenId={bytesToBigint(nftIds[i])} /> -->
      </div>
    {/each}
  {/if}
</div>

<Modal bind:open={mintMoveModal}>
  <MintMove bind:open={mintMoveModal} {tokenId} />
</Modal>
