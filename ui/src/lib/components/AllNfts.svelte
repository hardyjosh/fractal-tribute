<script lang="ts">
  import Board from "$lib/components/Board.svelte";
  import MintMove from "$lib/components/MintMove.svelte";
  import { fetchNftIds } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { onMount } from "svelte";
  import { bytesToBigint } from "viem";

  let nftIds: Uint8Array[] = [];

  onMount(async () => {
    nftIds = await fetchNftIds();
  });
</script>

{#each nftIds as nftId}
  {#await $happ.getBoardFromTokenId(nftId)}
    <p>loading...</p>
  {:then board}
    {#if board}
      <Board {board} size="w-2 h-2" />
      <MintMove tokenId={bytesToBigint(nftId)} />
    {/if}
  {/await}
{/each}
