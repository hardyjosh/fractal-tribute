<script lang="ts">
  import { onMount } from "svelte";
  import { happ } from "$lib/stores";
  import type { GameMoveWithActionHash } from "$lib/types";
  import Board from "$lib/components/Board.svelte";
  import MintMove from "$lib/components/MintMove.svelte";

  let gameMoves: GameMoveWithActionHash[];

  onMount(async () => {
    gameMoves = await $happ.getAllMyGameMoves();
  });
</script>

{#if gameMoves}
  <div class="flex flex-row gap-x-2">
    {#each gameMoves as gameMove}
      {#await $happ.getBoardAtMove(gameMove.actionHash)}
        <p>loading...</p>
      {:then board}
        <Board {board} size="w-2 h-2" />
        <MintMove move={gameMove.actionHash} />
      {:catch error}
        <p>{error.message}</p>
      {/await}
    {/each}
  </div>
{/if}
