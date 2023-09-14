<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata } from "$lib/types";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import { Heading, Button, Modal, Spinner } from "flowbite-svelte";
  import no_moves from "$lib/assets/no_moves.svg";
  import type { ActionHash } from "@holochain/client";
  import { fetchNftIds, actionHashAndAccountToTokenId } from "$lib/helpers";
  import { bytesToHex, type Hex } from "viem";

  let boards: BoardWithMetadata[];
  let nftIds: { id: Uint8Array; supply: number }[];
  let key: Hex;

  let ready: boolean = false;

  onMount(async () => {
    key = await $happ.getEvmAddress();
    await updateNftIds();
    await updateMyBoards();
    ready = true;
  });

  export const updateMyBoards = async () => {
    boards = await $happ.getBoardsFromAllMyMoves();
    boards.reverse();
  };

  export const updateNftIds = async () => {
    nftIds = await fetchNftIds();
  };

  let creationHash: ActionHash;
  let openModal = false;

  const snapshotMove = (action: ActionHash) => {
    creationHash = action;
    openModal = true;
  };
</script>

<Heading tag="h4" class="font-pixel">Your moves</Heading>
{#if ready}
  {#if boards?.length}
    <div in:fade class="flex overflow-scroll gap-4">
      {#each boards as board}
        <div class="flex flex-col gap-y-2 snap-start basis-1/5-gap-4 flex-none">
          <img
            alt="game board"
            class="border-2 border-black rounded-md"
            src={board.svg}
          />
          {#if key && nftIds.find((nft) => bytesToHex(nft.id) == bytesToHex(actionHashAndAccountToTokenId(board.creationHash, key)))}
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
    </div>
  {/if}
{:else}
  <div
    class="w-full rounded-lg border-2 border-black flex flex-col gap-y-2 items-center justify-center h-80"
  >
    <Spinner />
  </div>
{/if}

<Modal bind:open={openModal}>
  <SnapshotMove move={creationHash} bind:open={openModal} />
</Modal>
