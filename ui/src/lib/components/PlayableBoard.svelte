<script lang="ts">
  import { account } from "svelte-wagmi-stores";
  import { onMount } from "svelte";
  import BoardComp from "$lib/components/Board.svelte";
  import type { Board, GameMove } from "$lib/types";
  import { happ } from "$lib/stores";
  import { mergeGameMoveIntoBoard } from "$lib/helpers";
  import Palette from "$lib/components/Palette.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { Button, Modal, Heading } from "flowbite-svelte";
  import { UndoOutline, RedoOutline } from "flowbite-svelte-icons";
  import { addToast } from "$lib/components/toasts";
  import type { ActionHash } from "@holochain/client";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";

  enum MoveStatus {
    Thinking,
    NotBinded,
    Ready,
    Cooldown,
  }

  let moveStatus: MoveStatus = MoveStatus.Thinking;

  let move: GameMove = {
    changes: [],
  };
  let undoneChanges = [];
  $: allMovesMade = move.changes.length == 10;

  let board: Board;
  let mergedBoard: Board;

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

    // Clear the undone changes since we're making a new change
    undoneChanges = [];

    move = move;
  };

  const undo = () => {
    const change = move.changes.pop();
    if (change) {
      undoneChanges.push(change);
    }
    move = move;
    undoneChanges = undoneChanges;
  };

  const redo = () => {
    const change = undoneChanges.pop();
    if (change) {
      move.changes.push(change);
    }
    undoneChanges = undoneChanges;
    move = move;
  };

  const saveMove = async () => {
    try {
      const record = await $happ.createGameMove(move);
      savedMoveActionHash = record.signed_action.hashed.hash;
      await getBoard();
      move = {
        changes: [],
      };
      undoneChanges = [];
      snapshotMove = false;
      promptSnapshot = true;
    } catch (e) {
      addToast("error", `Error saving move: ${e?.data?.data || e}`);
    }
  };

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
  };

  onMount(async () => {
    getBoard();
    const key = await $happ.getEvmAddress();
    moveStatus = key ? MoveStatus.Ready : MoveStatus.NotBinded;
  });

  // modal
  let promptSnapshot = false;
  let snapshotMove = false;
  let savedMoveActionHash: ActionHash;
</script>

<div class="gap-x-4 items-stretch grid grid-cols-5">
  <div class="col-span-3">
    <BoardComp
      board={mergedBoard}
      bind:brush
      on:tileClick={handleTileClick}
      notAllowed={allMovesMade}
      readOnly={moveStatus !== MoveStatus.Ready || !$account?.isConnected}
    />
  </div>
  <div class="col-span-2">
    {#if moveStatus == MoveStatus.NotBinded || !$account?.isConnected}
      <CreateEvmKeyBinding
        on:evmKeyBindingCreated={() => {
          moveStatus = MoveStatus.Ready;
        }}
      />
    {:else if moveStatus == MoveStatus.Ready}
      <div class="p-4 border-2 border-black rounded-lg mb-4">
        You've made {move.changes.length}/10 changes.
      </div>
      <div class="flex gap-x-2 mb-4">
        <Button
          color="none"
          class="border-2 border-black grow"
          size="lg"
          disabled={!move.changes.length}
          on:click={undo}><RedoOutline class="mr-2 w-4" />Undo</Button
        >
        <Button
          color="none"
          class="border-2 border-black grow"
          size="lg"
          disabled={!undoneChanges?.length}
          on:click={redo}><UndoOutline class="mr-2 w-4" />Redo</Button
        >
        <Button
          class="bg-fractalorange border-2 border-black grow"
          size="lg"
          disabled={!move.changes.length}
          on:click={saveMove}>Save Move</Button
        >
      </div>
      <Palette bind:color bind:graphic_option />
    {/if}
  </div>
</div>

<Modal bind:open={promptSnapshot}>
  {#if !snapshotMove}
    <Heading tag="h4">Nice move!</Heading>
    <p>Would you like to mint a snapshot?</p>
    <p>
      Minting a snapshot allows you to earn some percent of the pools earning.
    </p>
    <p>
      If you change your mind later, you can mint a snapshot of any of your
      moves.
    </p>
    <div class="flex gap-x-2">
      <Button
        on:click={() => {
          promptSnapshot = false;
        }}
        color="none"
        class="border-2 border-black"
      >
        No thanks
      </Button>
      <Button
        on:click={() => {
          snapshotMove = true;
        }}
        class="bg-fractalorange border-2 border-black"
      >
        Mint
      </Button>
    </div>
  {:else}
    <SnapshotMove move={savedMoveActionHash} bind:open={promptSnapshot} />
  {/if}
</Modal>
