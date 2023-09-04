<script lang="ts">
  import { onMount } from "svelte";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata } from "$lib/types";
  import Board from "$lib/components/Board.svelte";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import { Heading, Button, Modal } from "flowbite-svelte";
  import no_moves from "$lib/assets/no_moves.svg";
  import type { ActionHash } from "@holochain/client";
  import { fetchNftIds, actionHashAndAccountToTokenId } from "$lib/helpers";
  import { bytesToHex, type Hex } from "viem";
  import ReadOnlyBoardSvg from "$lib/components/ReadOnlyBoardSvg.svelte";

  let boards: BoardWithMetadata[];
  let nftIds: Uint8Array[];
  let key: Hex;

  onMount(async () => {
    key = await $happ.getEvmAddress();
    nftIds = await fetchNftIds();
    boards = await $happ.getBoardsFromAllMyMoves();
  });

  let creationHash: ActionHash;
  let openModal = false;

  const snapshotMove = (action: ActionHash) => {
    creationHash = action;
    openModal = true;
  };
</script>

<Heading tag="h4" class="font-pixel">Your moves</Heading>
{#if boards?.length && key}
  <div class="flex overflow-scroll gap-4">
    {#each boards as board}
      {@const tokenId = actionHashAndAccountToTokenId(board.creationHash, key)}
      <div class="flex flex-col gap-y-2 snap-start basis-1/5-gap-4 flex-none">
        <!-- <Board readOnly board={board.board} /> -->
        <ReadOnlyBoardSvg board={board.board} />
        {#if nftIds.find((id) => bytesToHex(id) == bytesToHex(tokenId))}
          <Button
            on:click={() => {
              snapshotMove(board.creationHash);
            }}
            disabled
            class="bg-fractalorange border-2 border-black"
            >Snapshot created</Button
          >
        {:else}
          <Button
            on:click={() => {
              snapshotMove(board.creationHash);
            }}
            class="bg-fractalorange border-2 border-black"
            >Create snapshot</Button
          >
        {/if}
      </div>
    {/each}
  </div>
{:else}
  <div
    class="w-full rounded-lg border-2 border-black flex flex-col gap-y-2 items-center justify-center h-80"
  >
    <img src={no_moves} alt="no snapshots" />
    <p class="text-2xl font-semibold">No moves yet</p>
    <p>After you make your first move, it will appear here.</p>
  </div>{/if}

<Modal bind:open={openModal}>
  <SnapshotMove move={creationHash} bind:open={openModal} />
</Modal>
