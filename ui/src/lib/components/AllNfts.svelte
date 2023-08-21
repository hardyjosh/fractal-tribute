<script lang="ts">
  import Board from "$lib/components/Board.svelte";
  import { fetchNftIds, happ } from "$lib/stores";
  import { onMount } from "svelte";

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
    {/if}
  {/await}
{/each}
