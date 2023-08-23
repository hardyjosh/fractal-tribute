<script lang="ts">
  import { onMount } from "svelte";
  import { happ } from "$lib/stores";
  import type { GameMoveWithActionHash } from "$lib/types";
  import Board from "$lib/components/Board.svelte";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import { Heading } from "flowbite-svelte";

  let gameMoves: GameMoveWithActionHash[];

  onMount(async () => {
    gameMoves = await $happ.getAllMyGameMoves();
  });
</script>

<Heading tag="h4">Your moves</Heading>
{#if gameMoves?.length}
  <div class="flex flex-row gap-x-2 overflow-x-scroll snap-x">
    {#each gameMoves as gameMove}
      {#await $happ.getBoardAtMove(gameMove.actionHash)}
        <p>loading...</p>
      {:then board}
        <div class="flex flex-col gap-y-4 snap-start">
          <Board {board} size="w-2 h-2" />
          <SnapshotMove move={gameMove.actionHash} />
        </div>
      {:catch error}
        <p>{error.message}</p>
      {/await}
    {/each}
  </div>
{:else}
  No moves yet.
{/if}
