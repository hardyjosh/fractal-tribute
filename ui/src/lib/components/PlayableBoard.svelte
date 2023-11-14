<script lang="ts">
  import {
    onMount,
    onDestroy,
    createEventDispatcher,
    getContext,
  } from "svelte";
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
  import { Button, Modal, Spinner, Heading } from "flowbite-svelte";
  import {
    UndoOutline,
    RedoOutline,
    ArrowsRepeatOutline,
  } from "flowbite-svelte-icons";
  import { addToast } from "$lib/components/toasts";
  import type { ActionHash } from "@holochain/client";
  import SnapshotMove from "$lib/components/SnapshotMove.svelte";
  import BoardNew from "$lib/components/BoardNew.svelte";
  import { CHANGES_PER_MOVE } from "$lib/constants";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";
  import { formatCountdown } from "$lib/stores/countdown";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

  const dispatch = createEventDispatcher();
  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

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
  let png: string;

  let wrapper: HTMLDivElement;

  let palette: Palette;
  let brush: Brush;
  let color: Color;
  let graphic_option: number;
  let brushTool: BrushTool;

  let hideGrid: boolean;

  let saving;
  let saved;

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
      const graphic_option =
        mergedBoard[event.detail.x][event.detail.y]?.graphic_option;
      if (!color) return;
      palette.setColor(color);
      palette.setGraphicOption(graphic_option);
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
      await new Promise((resolve) => setTimeout(resolve, 200));
      saving = false;
      if (promptSnapshot) snapshotMove = true;
      move = {
        changes: [],
      };
      undoneChanges = [];
      saved = true;
    } catch (e) {
      addToast(
        "error",
        `Error saving move: ${e?.message || e?.data?.data || e}`
      );
    }
  };

  const getBoard = async () => {
    board = await $happ.getLatestBoard();
    png = await $happ.boardToPng(board.board, "Large");
  };

  let refreshingBoard = false;

  const refreshBoard = async () => {
    if (refreshingBoard) return;
    refreshingBoard = true;
    const delay = new Promise((resolve) => setTimeout(resolve, 500));
    await Promise.all([getBoard, delay]);
    refreshingBoard = false;
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
      {png}
      {allMovesMade}
      {board}
      {brush}
      {move}
      {hideGrid}
      on:tileClick={handleTileClick}
    />
  </div>
  {#if $countdown?.timeRemaining}
    <div class="col-span-2">
      {#if moveStatus == MoveStatus.Ready}
        <div class="p-4 border-2 border-black rounded-lg mb-4">
          <En>
            You've made {move.changes.length}/{CHANGES_PER_MOVE} changes.
          </En>
          <Tr>
            Şu ana kadar {move.changes.length}/{CHANGES_PER_MOVE} değişiklik yaptın.
          </Tr>
        </div>
        <div class="flex gap-x-2 mb-4">
          <Button
            color="none"
            class="border-2 border-black grow"
            size="lg"
            disabled={!move.changes.length}
            on:click={undo}
            ><RedoOutline class="mr-2 w-4" /><En>Undo</En><Tr
              >Geri al
            </Tr></Button
          >
          <Button
            color="none"
            class="border-2 border-black grow"
            size="lg"
            disabled={!undoneChanges?.length}
            on:click={redo}
            ><UndoOutline class="mr-2 w-4" /><En>Redo</En><Tr>Yinele</Tr
            ></Button
          >
          <Button
            class="bg-fractalorange border-2 border-black grow w-1/4"
            size="lg"
            disabled={!move.changes.length || saving}
            on:click={() => saveMove(false)}
          >
            {#if saving}
              <Spinner size="5" class="mr-2" />
              <En>Saving</En><Tr>Kaydediliyor...</Tr>
            {:else if saved && !move.changes.length}
              <En>Saved</En><Tr>Kaydedildi</Tr>
            {:else}
              <En>Save Move</En><Tr>Hamleyi Kaydet</Tr>
            {/if}
          </Button>
          <Button
            class="bg-fractalorange border-2 border-black grow"
            size="lg"
            disabled={!move.changes.length || saving}
            on:click={() => saveMove(true)}
            ><En>Save & Mint Snapshot</En><Tr
              >Görüntüyü Kaydet & Bas (Mintle)
            </Tr></Button
          >
        </div>
        <Palette
          bind:color
          bind:graphic_option
          bind:brushTool
          bind:this={palette}
          bind:hideGrid
          {refreshBoard}
          {refreshingBoard}
        />
      {/if}
    </div>
  {:else}
    <div class="col-span-2 h-full">
      <div
        class="p-4 border-2 border-black rounded-lg mb-4 flex flex-col items-center text-center text-xl gap-y-6 py-12"
      >
        <Heading tag="h4"
          ><En>This game has ended!</En><Tr>Bu oyun sona erdi!</Tr></Heading
        >
        <p>
          <En
            >This means no new moves can be made and no new snapshots can be
            created.</En
          >
          <Tr
            >Bu, yeni hamlelerin yapılamayacağı ve yeni anlık görüntülerin
            oluşturulamayacağı anlamına gelir.</Tr
          >
        </p>
        {#if $snapshotEndCountdown.timeRemaining}
          <p class="font-bold">
            <En
              >However, you are free to collect existing snapshot NFTs until
              claims open in {formatCountdown($snapshotEndCountdown)}</En
            >
            <Tr
              >Ancak, iddialar açılana kadar mevcut anlık görüntü NFT'lerini
              toplamakta özgürsünüz {formatCountdown($snapshotEndCountdown)}
            </Tr>
          </p>
        {:else}
          <p>
            <En
              >Claims are now open! Head over to the leaderboard to see what
              your share of the pool is and claim.</En
            >
            <Tr
              >İddialar şimdi açık! Havuzunuzun payını görmek ve talep etmek
              için liderlik tablosuna gidin.
            </Tr>
          </p>
        {/if}
      </div>
    </div>
  {/if}
</div>

<Modal size="sm" bind:open={saving}>
  <div class="flex flex-col items-center justify-center p-4 gap-y-4">
    <Spinner size="10" />
    <span class="text-lg"
      ><En>Saving move...</En><Tr>Hamleyi kaydediyor</Tr></span
    >
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
