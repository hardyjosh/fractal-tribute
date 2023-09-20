<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { actionHashAndAccountToTokenId } from "$lib/helpers";
  import type { BoardWithMetadata } from "$lib/types";
  import type { ActionHash } from "@holochain/client";
  import { Button } from "flowbite-svelte";
  import { bytesToHex, type Hex } from "viem";
  import { happ } from "$lib/stores";
  import { Spinner } from "flowbite-svelte";
  import { nfts } from "$lib/stores/nfts";

  const dispatch = createEventDispatcher();

  export let actionHash: ActionHash;
  export let key: Hex;

  let board: BoardWithMetadata;

  onMount(async () => {
    console.log("should be requesting the board");
    board = await $happ.getBoardAtMove(actionHash);
  });
</script>

<div class="flex flex-col gap-y-2 snap-start basis-1/5-gap-4 flex-none">
  <div
    class="aspect-square w-full border-2 border-black rounded-md flex flex-col items-center justify-center"
  >
    {#if board}
      <img alt="game board" class="h-full" src={board.svg} />
    {:else}
      <Spinner />
    {/if}
  </div>
  {#if key && $nfts?.find((nft) => bytesToHex(nft.id) == bytesToHex(actionHashAndAccountToTokenId(actionHash, key)))}
    <Button
      on:click={() => {
        dispatch("snapshot");
        // snapshotMove(board.creationHash);
      }}
      disabled
      class="bg-fractalorange border-2 border-black">Snapshot created</Button
    >
  {:else}
    <Button
      on:click={() => {
        dispatch("snapshot");
        // snapshotMove(board.creationHash);
      }}
      class="bg-fractalorange border-2 border-black">Create snapshot</Button
    >
  {/if}
</div>
