<script lang="ts">
  import AllMyMoves from "$lib/components/AllMyMoves.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import BoardComp from "$lib/components/Board.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { generateRandomGameMove } from "$lib/helpers/game-move";
  import { happ } from "$lib/stores";
  import type { Board } from "$lib/types";
  import { Button } from "flowbite-svelte";

  let board: Board;

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
  };
</script>

<CreateEvmKeyBinding />

<Button
  on:click={() => {
    $happ.createGameMove(generateRandomGameMove());
  }}>Create a random game move</Button
>

<Button on:click={getBoard}>Get latest board</Button>

<BoardComp {board} />

<AllMyMoves />

<AllNfts />
