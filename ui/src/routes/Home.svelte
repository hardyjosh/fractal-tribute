<script lang="ts">
  import AllMyMovesAlt from "$lib/components/AllMyMovesAlt.svelte";
  import AllNfts from "$lib/components/AllNfts.svelte";
  import PlayableBoard from "$lib/components/PlayableBoard.svelte";
  import { nfts } from "$lib/stores/nfts";
  import { waitForTransaction } from "@wagmi/core";
  import { Heading } from "flowbite-svelte";
  import RandomGameMoves from "$lib/components/RandomGameMoves.svelte";
  import En from "$lib/components/i18n/En.svelte";
  import Tr from "$lib/components/i18n/Tr.svelte";

  let allMyMoves: AllMyMovesAlt;
</script>

<div class="flex flex-col w-full gap-y-10 will-change-auto">
  <PlayableBoard
    on:moveSaved={allMyMoves.updateMyBoards}
    on:snapshotMinted={({ detail: hash }) => {
      waitForTransaction({ hash, confirmations: 3 }).then(() => {
        allMyMoves.updateMyBoards();
        nfts.fetch();
      });
    }}
  />

  <AllMyMovesAlt bind:this={allMyMoves} />
  <!-- <RandomGameMoves /> -->
</div>
