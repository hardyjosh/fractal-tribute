<script lang="ts">
  import AllMyMoves from "$lib/components/AllMyMoves.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import BuildParticipation from "$lib/components/Participation.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
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

  <!-- 
  <Button
    on:click={() => {
      $happ.createGameMove(generateRandomGameMove());
    }}>Create a random game move</Button
  > -->

  <AllNfts />

  <AllMyMoves bind:this={allMyMoves} />

  <!-- <BuildParticipation /> -->
</div>
