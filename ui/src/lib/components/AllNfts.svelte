<script lang="ts">
  import Board from "$lib/components/Board.svelte";
  import { fetchNftIds, formatAddress } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { Button, Heading, Modal } from "flowbite-svelte";
  import { onMount, onDestroy } from "svelte";
  import type { BoardWithMetadataAndId } from "$lib/types";
  import { encodeHashToBase64 } from "@holochain/client";
  import no_snapshots from "$lib/assets/no_snapshots.svg";
  import MintMove from "$lib/components/MintMove.svelte";
  import { hexToBigInt, type Hex, bytesToHex } from "viem";

  let nftIds: { id: Uint8Array; supply: number }[] = [];
  let boards: BoardWithMetadataAndId[] = [];

  let pollInterval;

  onMount(async () => {
    nftIds = await fetchNftIds();
    boards = await $happ.getBoardsFromTokenIds(nftIds.map((nft) => nft.id));
    pollInterval = setInterval(async () => {
      boards = await $happ.getBoardsFromTokenIds(nftIds.map((nft) => nft.id));
    }, 20000);
  });

  onDestroy(() => {
    clearInterval(pollInterval);
  });

  let mintMoveModal = false;
  let tokenId: bigint;

  const mintMove = (id: Hex) => {
    mintMoveModal = true;
    tokenId = hexToBigInt(id);
  };
</script>

<Heading tag="h4" class="font-pixel">Latest snapshots</Heading>
<div class="flex overflow-scroll gap-4">
  {#if !boards || boards?.length == 0}
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
    {#each boards as board, i}
      <div class="flex flex-col gap-y-2 basis-1/5-gap-4 flex-none snap-start">
        <img
          class="border-2 border-black"
          src={board.boardWithMetadata.svg}
          alt="game board"
        />
        <div
          class="rounded-md border-black border-2 flex gap-x-2 p-2 justify-between items-center w-full"
        >
          <div class="flex flex-col">
            <span class="text-gray-500 font-light"
              >{formatAddress(
                encodeHashToBase64(board.boardWithMetadata.creator)
              )}</span
            >
            <span
              >{nftIds.find((nft) => bytesToHex(nft.id) == board.id).supply} minted</span
            >
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
