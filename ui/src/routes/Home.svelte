<script lang="ts">
  import AllMyMoves from "$lib/components/AllMyMoves.svelte";
  import AllMyMovesAlt from "$lib/components/AllMyMovesAlt.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import BuildParticipation from "$lib/components/Participation.svelte";
  import PlayableBoard from "$lib/components/PlayableBoard.svelte";

  let allMyMoves: AllMyMoves;
</script>

<div class="flex flex-col w-full gap-y-10">
  <PlayableBoard
    on:moveSaved={allMyMoves.updateMyBoards}
    on:snapshotMinted={() => {
      allMyMoves.updateMyBoards();
      allMyMoves.updateNftIds();
    }}
  />
  <!-- <PlayableBoard /> -->
  <AllNfts />

  <!-- <AllMyMoves bind:this={allMyMoves} /> -->
  <AllMyMovesAlt bind:this={allMyMoves} />

  <!-- <BuildParticipation /> -->
</div>
