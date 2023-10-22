<script lang="ts">
  import { happ } from "$lib/stores";
  import AllMyMovesAlt from "$lib/components/AllMyMovesAlt.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import BuildParticipation from "$lib/components/Participation.svelte";
  import PlayableBoard from "$lib/components/PlayableBoard.svelte";
  import { nfts } from "$lib/stores/nfts";
  import { waitForTransaction } from "@wagmi/core";
  import { Heading } from "flowbite-svelte";

  let allMyMoves: AllMyMovesAlt;
</script>

<div
  class="flex flex-col w-full gap-y-10 will-change-auto"
  style="transform: translateZ(0);"
>
  <PlayableBoard
    on:moveSaved={allMyMoves.updateMyBoards}
    on:snapshotMinted={({ detail: hash }) => {
      waitForTransaction({ hash, confirmations: 3 }).then(() => {
        allMyMoves.updateMyBoards();
        nfts.fetch();
      });
    }}
  />
  <div class="flex flex-col gap-y-2">
    <Heading tag="h3">Latest snapshots</Heading>
    <p class="text-lg">Vote for your favourite snapshots by minting them</p>
  </div>

  <AllNfts />
  <AllMyMovesAlt bind:this={allMyMoves} />
  <RandomGameMoves />
</div>
