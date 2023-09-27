<script lang="ts">
  import AllMyMoves from "$lib/components/AllMyMoves.svelte";
  import AllMyMovesAlt from "$lib/components/AllMyMovesAlt.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import BuildParticipation from "$lib/components/Participation.svelte";
  import PlayableBoard from "$lib/components/PlayableBoard.svelte";
  import { nfts } from "$lib/stores/nfts";
  import { waitForTransaction } from "@wagmi/core";

  let allMyMoves: AllMyMovesAlt;
</script>

<div class="flex flex-col w-full gap-y-10">
  <PlayableBoard
    on:moveSaved={allMyMoves.updateMyBoards}
    on:snapshotMinted={({ detail: hash }) => {
      console.log({ hash });
      waitForTransaction({ hash, confirmations: 3 }).then(() => {
        allMyMoves.updateMyBoards();
        nfts.fetch();
      });
    }}
  />
  <!-- <PlayableBoard /> -->
  <AllNfts />

  <!-- <AllMyMoves bind:this={allMyMoves} /> -->
  <AllMyMovesAlt bind:this={allMyMoves} />

  <!-- <BuildParticipation /> -->
</div>
