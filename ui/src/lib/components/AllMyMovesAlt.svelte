<script lang="ts">
  import { nfts } from "$lib/stores/nfts";
  import { onMount } from "svelte";
  import { happ } from "$lib/stores";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import { Heading, Modal, Spinner } from "flowbite-svelte";
  import no_moves from "$lib/assets/no_moves.svg";
  import type { ActionHash } from "@holochain/client";
  import { bytesToHex, type Hex } from "viem";
  import MyMoveCard from "$lib/components/MyMoveCard.svelte";
  import IntersectionObserver from "svelte-intersection-observer";
  import Tr from "$lib/components/i18n/Tr.svelte";
  import En from "$lib/components/i18n/En.svelte";

  let moveActions: ActionHash[] = [];
  let key: Hex;
  let wrappers: HTMLElement[] = [];

  onMount(async () => {
    key = await $happ.getEvmAddress();
  });

  export const updateMyBoards = async () => {
    const moves = await $happ.getAllMyGameMoves();
    const _moveActions = moves.map((move) => move.actionHash);
    _moveActions.reverse();
    moveActions = _moveActions;
  };

  $: if ($nfts) updateMyBoards();

  let creationHash: ActionHash;
  let openModal = false;

  const snapshotMove = (action: ActionHash) => {
    creationHash = action;
    openModal = true;
  };

  $: ready = moveActions && $nfts;
</script>

<div class="flex flex-col gap-y-2">
  <Heading tag="h3"><En>Your moves</En><Tr>Senin hamlelerin</Tr></Heading>
  <p class="text-lg">
    <En>Make a snapshot onchain for any of your moves by minting them</En><Tr
      >Hamlelerinden herhangi birini basarak (mintleyerek) zincir üzerinde bir
      anlık görüntü oluştur</Tr
    >
  </p>
</div>

{#if ready}
  {#if moveActions?.length}
    <div class="flex overflow-scroll gap-4">
      {#each moveActions as action (action)}
        <IntersectionObserver
          once
          element={wrappers[bytesToHex(action)]}
          let:intersecting
        >
          <div
            bind:this={wrappers[bytesToHex(action)]}
            class="flex flex-col gap-y-2 snap-start basis-1/5-gap-4 flex-none"
          >
            {#if intersecting}
              <MyMoveCard
                actionHash={action}
                {key}
                on:snapshot={() => {
                  snapshotMove(action);
                }}
              />
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
      <p class="text-2xl font-semibold">
        <En>No moves yet</En><Tr>Henüz hiçbir hamle yok</Tr>
      </p>
      <p>
        <En>After you make your first move, it will appear here.</En><Tr
          >İlk hamleni yaptıktan sonra burada görünecek</Tr
        >
      </p>
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
