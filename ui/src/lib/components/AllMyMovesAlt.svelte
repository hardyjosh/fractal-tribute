<script lang="ts">
  import { fade } from "svelte/transition";
  import { onMount, onDestroy } from "svelte";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadata } from "$lib/types";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import { Heading, Button, Modal, Spinner } from "flowbite-svelte";
  import no_moves from "$lib/assets/no_moves.svg";
  import type { ActionHash } from "@holochain/client";
  import { fetchNftIds, actionHashAndAccountToTokenId } from "$lib/helpers";
  import { bytesToHex, type Hex } from "viem";
  import MyMoveCard from "$lib/components/MyMoveCard.svelte";
  import IntersectionObserver from "svelte-intersection-observer";

  let nftIds: { id: Uint8Array; supply: number }[];
  let moveActions: ActionHash[] = [];
  let key: Hex;
  let wrappers: HTMLElement[] = [];

  let ready: boolean = false;
  let interval;

  onMount(async () => {
    key = await $happ.getEvmAddress();
    await updateNftIds();
    await updateMyBoards();
    ready = true;
    interval = setInterval(async () => {
      await updateMyBoards();
      await updateNftIds();
    }, 10000);
  });

  onDestroy(() => {
    clearInterval(interval);
  });

  export const updateMyBoards = async () => {
    const moves = await $happ.getAllMyGameMoves();
    const _moveActions = moves.map((move) => move.actionHash);
    _moveActions.reverse();
    moveActions = _moveActions;
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

<div class="flex flex-col gap-y-2">
  <Heading tag="h4" class="font-pixel">Your moves</Heading>
  <p class="text-lg">
    Make a snapshot onchain for any of your moves by minting them
  </p>
</div>

{#if ready}
  {#if moveActions?.length}
    <div class="flex overflow-scroll gap-4">
      {#each moveActions as action, i}
        <IntersectionObserver element={wrappers[i]} let:intersecting>
          <div
            bind:this={wrappers[i]}
            class="flex flex-col gap-y-2 snap-start basis-1/5-gap-4 flex-none"
          >
            {#if intersecting}
              <MyMoveCard actionHash={action} {key} />
            {/if}
          </div>
        </IntersectionObserver>
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
