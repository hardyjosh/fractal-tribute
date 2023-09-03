<script lang="ts">
  import Board from "$lib/components/Board.svelte";
  import MintMove from "$lib/components/MintMove.svelte";
  import { fetchNftIds, formatAddress } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { Button, Heading } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { bytesToBigint } from "viem";
  import type { BoardWithMetadataAndId } from "$lib/types";
  import { encodeHashToBase64 } from "@holochain/client";
  import no_snapshots from "$lib/assets/no_snapshots.svg";

  let nftIds: Uint8Array[] = [];
  let boards: BoardWithMetadataAndId[] = [];

  onMount(async () => {
    nftIds = await fetchNftIds();
    boards = await $happ.getBoardsFromTokenIds(nftIds);
  });
</script>

<Heading tag="h4" class="font-pixel">Latest snapshots</Heading>
<div class="flex gap-x-4">
  {#if !boards || boards?.length == 0}
    <div
      class="w-full rounded-lg border-2 border-black flex flex-col gap-y-2 items-center justify-center h-60"
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
      <div class="flex flex-col gap-y-4">
        <Board board={board.boardWithMetadata.board} size="w-2 h-2" />
        <div
          class="rounded-md border-black border-2 flex gap-x-2 p-2 justify-between items-center w-full"
        >
          <div class="flex flex-col">
            <span class="text-gray-500 font-light"
              >{formatAddress(
                encodeHashToBase64(board.boardWithMetadata.creator)
              )}</span
            >
            <span>X minted</span>
          </div>
          <Button class="!bg-primary-500 border-black border-2" size="sm"
            >Mint</Button
          >
        </div>
        <!-- <MintMove tokenId={bytesToBigint(nftIds[i])} /> -->
      </div>
    {/each}
  {/if}
</div>
