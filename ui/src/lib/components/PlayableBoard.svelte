<script lang="ts">
  import { account } from "svelte-wagmi-stores";
  import { onMount, onDestroy, createEventDispatcher } from "svelte";
  import BoardComp from "$lib/components/Board.svelte";
  import type { Board, BoardWithMetadata, GameMove } from "$lib/types";
  import { happ } from "$lib/stores";
  import { mergeGameMoveIntoBoard } from "$lib/helpers";
  import Palette from "$lib/components/Palette.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import { Button, Modal, Heading, Spinner } from "flowbite-svelte";
  import { UndoOutline, RedoOutline } from "flowbite-svelte-icons";
  import { addToast } from "$lib/components/toasts";
  import type { ActionHash } from "@holochain/client";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import BoardNew from "$lib/components/BoardNew.svelte";
  import { CHANGES_PER_MOVE } from "$lib/constants";

  const dispatch = createEventDispatcher();

  enum MoveStatus {
    Thinking,
    Ready,
    Cooldown,
  }

  let moveStatus: MoveStatus = MoveStatus.Ready;

  let move: GameMove = {
    changes: [],
  };
  let undoneChanges = [];
  $: allMovesMade = move.changes.length == CHANGES_PER_MOVE;

  let board: BoardWithMetadata;
  let mergedBoard: Board;

  let wrapper: HTMLDivElement;

  let palette;
  let color;
  let graphic_option;
  let eyeDropper;

  let saving;

  $: brush = { color, graphic_option, eyeDropper };
  $: if (board) mergedBoard = mergeGameMoveIntoBoard(board.board, move);

  const handleTileClick = (event: CustomEvent<{ x: number; y: number }>) => {
    // if we're just picking a colour, update the palette
    if (eyeDropper) {
      const color = mergedBoard[event.detail.x][event.detail.y]?.color;
      if (!color) return;
      palette.setColor(color);
      return;
    }

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
      graphic_option: brush.graphic_option,
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
      snapshotMove = false;
      saving = true;
      await getBoard();
      promptSnapshot = true;
      dispatch("moveSaved");
      saving = false;
      move = {
        changes: [],
      };
      undoneChanges = [];
    } catch (e) {
      addToast("error", `Error saving move: ${e?.data?.data || e}`);
    }
  };

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
  };

  let pollingInterval;

  onMount(async () => {
    pollingInterval = setInterval(getBoard, 10000);
    getBoard();
  });

  onDestroy(() => {
    clearInterval(pollingInterval);
  });

  // modal
  let promptSnapshot = false;
  let snapshotMove = false;
  let savedMoveActionHash: ActionHash;

  // $: console.log("brush in playable board", brush);
</script>

<div class="gap-x-4 items-stretch grid grid-cols-5">
  <div bind:this={wrapper} class="col-span-3 relative">
    <!-- <BoardComp board={mergedBoard} bind:brush on:tileClick={handleTileClick} /> -->
    {#if board}
      <BoardNew
        {allMovesMade}
        {board}
        {brush}
        {move}
        {eyeDropper}
        on:tileClick={handleTileClick}
      />
      <!-- <img class="absolute inset-0" src={board.svg} /> -->
    {/if}
  </div>
  <div class="col-span-2">
    {#if moveStatus == MoveStatus.Ready}
      <div class="p-4 border-2 border-black rounded-lg mb-4">
        You've made {move.changes.length}/{CHANGES_PER_MOVE} changes.
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
          disabled={!move.changes.length || saving}
          on:click={saveMove}
          >{#if !saving}Save Move{:else}<Spinner size="4" class="mr-2" /> Saving...{/if}</Button
        >
      </div>
      <Palette
        bind:color
        bind:graphic_option
        bind:eyeDropper
        bind:this={palette}
      />
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
    <SnapshotMove
      move={savedMoveActionHash}
      bind:open={promptSnapshot}
      on:snapshotMinted
    />
  {/if}
</Modal>
