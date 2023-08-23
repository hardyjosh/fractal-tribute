<script lang="ts">
  import { onMount } from "svelte";
  import BoardComp from "$lib/components/Board.svelte";
  import type { Board, GameMove } from "$lib/types";
  import { happ } from "$lib/stores";
  import { mergeGameMoveIntoBoard } from "$lib/helpers";
  import Palette from "$lib/components/Palette.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { Button } from "flowbite-svelte";

  let move: GameMove = {
    changes: [],
  };
  $: allMovesMade = move.changes.length == 10;

  let board: Board;
  let mergedBoard: Board;
  $: console.log(mergedBoard);

  let color;
  let graphic_option;

  $: brush = { color, graphic_option };

  $: if (board) mergedBoard = mergeGameMoveIntoBoard(board, move);

  const handleTileClick = (event: CustomEvent<{ x: number; y: number }>) => {
    // if we've already placed this tile, don't do anything
    if (
      move.changes.find(
        (change) => change.x == event.detail.x && change.y == event.detail.y
      )
    )
      return;

    // if we've made all our moves, don't do anything
    if (allMovesMade) return;

    // otherwise, add the change to the move
    move.changes.push({
      x: event.detail.y,
      y: event.detail.x,
      color: {
        r: brush.color.r,
        g: brush.color.g,
        b: brush.color.b,
      },
      graphic_option: 0,
    });
    move = move;
  };

  const undo = () => {
    move.changes.pop();
    move = move;
  };

  const saveMove = async () => {
    await $happ.createGameMove(move);
    await getBoard();
    move = {
      changes: [],
    };
  };

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
  };

  onMount(() => {
    getBoard();
  });
</script>

<div class="grid grid-cols-3">
  <div class="col-span-2">
    <BoardComp
      board={mergedBoard}
      bind:brush
      on:tileClick={handleTileClick}
      notAllowed={allMovesMade}
    />
  </div>
  <div>
    <CreateEvmKeyBinding />

    <div>
      You've made {move.changes.length}/10 changes.
    </div>
    <Palette bind:color bind:graphic_option />
    <Button on:click={undo}>Undo</Button>
    <Button on:click={saveMove}>Make Move</Button>
  </div>
</div>
