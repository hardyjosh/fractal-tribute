<script lang="ts">
  import Board from "$lib/components/Board.svelte";
  import MintMove from "$lib/components/MintMove.svelte";
  import { fetchNftIds } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { Heading } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { bytesToBigint } from "viem";
  import type { Board as BoardType } from "$lib/types";

  let nftIds: Uint8Array[] = [];
  let boards: BoardType[] = [];

  onMount(async () => {
    nftIds = await fetchNftIds();
    boards = (
      await Promise.all(
        nftIds.map((id) => $happ.getBoardFromTokenId(id).catch((e) => null))
      )
    ).filter((b) => b) as BoardType[];
  });
</script>

<Heading tag="h4">Latest snapshots</Heading>
<div class="flex gap-x-4">
  {#if !boards || boards?.length == 0}
    <p>No snapshots yet</p>
  {:else}
    {#each boards as board, i}
      <div class="flex flex-col gap-y-4">
        <Board {board} size="w-2 h-2" />
        <MintMove tokenId={bytesToBigint(nftIds[i])} />
      </div>
    {/each}
  {/if}
</div>
