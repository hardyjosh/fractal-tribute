<script lang="ts">
  import AllMyMoves from "$lib/components/AllMyMoves.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import CreateEvmKeyBinding from "$lib/components/CreateEvmKeyBinding.svelte";
  import PlayableBoard from "$lib/components/PlayableBoard.svelte";
</script>

<PlayableBoard />

<!-- 
<Button
  on:click={() => {
    $happ.createGameMove(generateRandomGameMove());
  }}>Create a random game move</Button
> -->

<AllNfts />

<AllMyMoves />
