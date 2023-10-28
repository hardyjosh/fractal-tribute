<script lang="ts">
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import { actionHashAndAccountToTokenId } from "$lib/helpers";
  import type { BoardWithMetadata } from "$lib/types";
  import type { ActionHash } from "@holochain/client";
  import { Button } from "flowbite-svelte";
  import { bytesToHex, type Hex } from "viem";
  import { happ } from "$lib/stores";
  import { Spinner } from "flowbite-svelte";
  import { nfts } from "$lib/stores/nfts";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";
  import CanvasBoard from "$lib/components/CanvasBoard.svelte";

  const dispatch = createEventDispatcher();
  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

  export let actionHash: ActionHash;
  export let key: Hex;

  let board: BoardWithMetadata;
  let png: string;

  onMount(async () => {
    board = await $happ.getBoardAtMove(actionHash);
    png = await $happ.svgToPng(board.completeSvg, 0.2);
  });
</script>

<div class="flex flex-col gap-y-2 snap-start basis-1/5-gap-4 flex-none">
  <div
    class="aspect-square w-full border-2 border-black rounded-md flex flex-col items-center justify-center relative"
  >
    {#if board?.board}
      <CanvasBoard board={board.board} scale={0.2} />
    {/if}
    <!-- {#if png}
      <img alt="game board" class="h-full" src={board.svg} />
      <img src={png} alt="board" />
      {@html board.svg}
    {:else}
      <Spinner />
    {/if} -->
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
      disabled={!$snapshotEndCountdown?.timeRemaining}
      on:click={() => {
        dispatch("snapshot");
        // snapshotMove(board.creationHash);
      }}
      class="bg-fractalorange border-2 border-black">Create snapshot</Button
    >
  {/if}
</div>
