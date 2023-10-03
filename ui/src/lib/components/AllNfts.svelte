<script lang="ts">
  import { fade } from "svelte/transition";
  import { formatAddress } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import { Button, Modal } from "flowbite-svelte";
  import type { BoardWithMetadataAndId } from "$lib/types";
  import { encodeHashToBase64 } from "@holochain/client";
  import no_snapshots from "$lib/assets/no_snapshots.svg";
  import MintMove from "$lib/components/MintMove.svelte";
  import { hexToBigInt, type Hex, bytesToHex } from "viem";
  import { nfts } from "$lib/stores/nfts";
  import Identicon from "$lib/components/Identicon.svelte";
  import { getContext } from "svelte";
  import { countdownContext, type CountdownContextType } from "$lib/contexts";

  const { countdown, snapshotEndCountdown } = getContext(
    countdownContext
  ) as CountdownContextType;

  export let wrap: boolean = false;

  let boards: BoardWithMetadataAndId[] = [];
  let boardsWithSupply: (BoardWithMetadataAndId & { supply: number })[];

  const prepareHappNfts = async () => {
    boards = await $happ.getBoardsFromTokenIds($nfts.map((nft) => nft.id));
    boardsWithSupply = boards.map((board) => {
      const nft = $nfts.find((nft) => bytesToHex(nft.id) == board.id);
      return { ...board, supply: nft.supply };
    });
    boardsWithSupply.sort((a, b) => b.supply - a.supply);
  };

  $: if ($nfts) prepareHappNfts();

  let mintMoveModal = false;
  let tokenId: bigint;

  const mintMove = (id: Hex) => {
    mintMoveModal = true;
    tokenId = hexToBigInt(id);
  };
</script>

{#if boardsWithSupply}
  {#if boardsWithSupply?.length == 0}
    <div
      in:fade|global={{ duration: 200 }}
      class="w-full rounded-lg border-2 border-black flex flex-col gap-2 items-center justify-center h-80"
    >
      <img src={no_snapshots} alt="no snapshots" />
      <p class="text-2xl font-semibold">No snapshots yet</p>
      <p>
        Snapshots from you and other players will appear here once minted
        onchain.
      </p>
    </div>
  {:else}
    <div
      in:fade|global={{ duration: 200 }}
      class:flex-wrap={wrap}
      class="flex overflow-scroll gap-4"
    >
      {#each boardsWithSupply as board, i}
        <div
          class="relative flex flex-col gap-y-2 flex-none snap-start basis-1/5-gap-4"
        >
          <div class="aspect-square border-2 border-black rounded-lg">
            {@html board.boardWithMetadata.svg}
          </div>
          <div
            class="rounded-lg border-black border-2 flex gap-x-2 p-4 justify-between items-center w-full"
          >
            <div class="flex flex-col">
              <div class="flex gap-x-2 items-center">
                <Identicon agentHash={board.boardWithMetadata.creator} />
                <div class="flex flex-col leading-none gap-y-1">
                  <span class="text-gray-500 w-28 overflow-ellipsis truncate">
                    {#await $happ.getProfile(board.boardWithMetadata.creator) then profile}
                      {profile.name}
                    {:catch error}
                      {formatAddress(
                        encodeHashToBase64(board.boardWithMetadata.creator)
                      )}
                    {/await}
                  </span>
                  <span>{board.supply} minted</span>
                </div>
              </div>
            </div>
            <div class="flex flex-col gap-y-1">
              <Button
                on:click={() => {
                  mintMove(board.id);
                }}
                disabled={$snapshotEndCountdown?.timeRemaining == 0}
                class="!bg-primary-500 border-black border-2"
                size="sm">Mint</Button
              >
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
{/if}

<Modal permanent bind:open={mintMoveModal}>
  <MintMove bind:open={mintMoveModal} {tokenId} />
</Modal>
