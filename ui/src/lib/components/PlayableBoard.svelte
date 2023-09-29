<script lang="ts">
  import { account } from "svelte-wagmi-stores";
  import { onMount, onDestroy, createEventDispatcher } from "svelte";
  import BoardComp from "$lib/components/Board.svelte";
  import type {
    Board,
    BoardWithMetadata,
    Brush,
    BrushTool,
    Color,
    GameMove,
    PixelChange,
  } from "$lib/types";
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

  let palette: Palette;
  let brush: Brush;
  let color: Color;
  let graphic_option: number;
  let brushTool: BrushTool;

  let hideGrid: boolean = false;

  let saving;

  $: brush = { color, graphic_option, brushTool };
  $: if (board) mergedBoard = mergeGameMoveIntoBoard(board.board, move);

  const handleTileClick = (event: CustomEvent<{ x: number; y: number }>) => {
    // see if we can find another change for this move, at this location
    const previousChangeIndex = move.changes.findIndex(
      (prevChange) =>
        event.detail.y == prevChange.x && event.detail.x == prevChange.y
    );

    // if we've made all our moves, don't do anything
    if (allMovesMade) return;

    // if we're just picking a colour, update the palette and don't do anything else
    if (brushTool == "eye-dropper") {
      const color = mergedBoard[event.detail.x][event.detail.y]?.color;
      if (!color) return;
      palette.setColor(color);
      return;
    }

    // otherwise...

    // if we found a previous change, remove it
    if (previousChangeIndex > -1) {
      move.changes.splice(previousChangeIndex, 1);
    }

    // start creating a new change
    let change: PixelChange;

    // if we're erasing and there's already a change at this location for this move
    if (brushTool == "eraser" && previousChangeIndex > -1) {
      change = null;
    } else if (brushTool == "eraser") {
      // otherwise, we're adding a new change of a white tile
      change = {
        x: event.detail.y,
        y: event.detail.x,
        color: {
          r: 255,
          g: 255,
          b: 255,
        },
        graphic_option: 32,
      };
    } else {
      // otherwise, place the current brush
      change = {
        x: event.detail.y,
        y: event.detail.x,
        color: {
          r: brush.color.r,
          g: brush.color.g,
          b: brush.color.b,
        },
        graphic_option: brush.graphic_option,
      };
    }

    // add the new change if there is one
    if (change) move.changes.push(change);

    // clear the undone changes since we've done something new
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

  const saveMove = async (promptSnapshot: boolean) => {
    try {
      const record = await $happ.createGameMove(move);
      savedMoveActionHash = record.signed_action.hashed.hash;
      saving = true;
      await getBoard();
      dispatch("moveSaved");
      await new Promise((resolve) => setTimeout(resolve, 500));
      saving = false;
      if (promptSnapshot) snapshotMove = true;
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
  let snapshotMove = false;
  let savedMoveActionHash: ActionHash;
</script>

<div class="gap-x-4 items-stretch grid grid-cols-5">
  <div bind:this={wrapper} class="col-span-3 relative">
    <BoardNew
      {allMovesMade}
      {board}
      {brush}
      {move}
      {hideGrid}
      on:tileClick={handleTileClick}
    />
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <button
      on:click={() => {
        hideGrid = !hideGrid;
      }}
    >
      {#if !hideGrid}
        Hide grid
      {:else}
        Show grid
      {/if}
    </button>
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
          on:click={() => saveMove(false)}>Save Move</Button
        >
        <Button
          class="bg-fractalorange border-2 border-black grow"
          size="lg"
          disabled={!move.changes.length || saving}
          on:click={() => saveMove(true)}>Save & Mint Snapshot</Button
        >
      </div>
      <Palette
        bind:color
        bind:graphic_option
        bind:brushTool
        bind:this={palette}
      />
    {/if}
  </div>
</div>

<Modal size="sm" bind:open={saving}>
  <div class="flex flex-col items-center justify-center p-4 gap-y-4">
    <Spinner size="10" />
    <span class="text-lg">Saving move...</span>
  </div>
</Modal>

<Modal bind:open={snapshotMove}>
  <SnapshotMove
    move={savedMoveActionHash}
    bind:open={snapshotMove}
    on:snapshotMinted
    isPostMove
  />
</Modal>
