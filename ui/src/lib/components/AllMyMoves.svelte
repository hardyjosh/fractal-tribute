<script lang="ts">
  import { onMount } from "svelte";
  import { generateRandomGameMove } from "$lib/game-move";
  import { happ } from "$lib/stores";
  import type { GameMoveWithActionHash } from "$lib/types";
  import { Button } from "flowbite-svelte";
  import Board from "$lib/components/Board.svelte";

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
      {:catch error}
        <p>{error.message}</p>
      {/await}
    {/each}
  </div>
{/if}
