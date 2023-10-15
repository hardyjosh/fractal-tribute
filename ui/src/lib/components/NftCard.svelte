<script lang="ts">
  import Identicon from "$lib/components/Identicon.svelte";
  import MintMove from "$lib/components/MintMove.svelte";
  import { formatAddress } from "$lib/helpers";
  import { happ } from "$lib/stores";
  import type { BoardWithMetadataAndId } from "$lib/types";
  import { encodeHashToBase64 } from "@holochain/client";
  import { Button, Modal } from "flowbite-svelte";
  import { hexToBigInt, type Hex } from "viem";

  export let board: BoardWithMetadataAndId & { supply: number };
  export let mintDisabled: boolean = false;
  let mintMoveModal = false;
  let tokenId: bigint;

  const mintMove = (id: Hex) => {
    mintMoveModal = true;
    tokenId = hexToBigInt(id);
  };
</script>

<div class="aspect-square border-2 border-black rounded-lg relaive">
  <!-- <img src={board.boardWithMeåtadata.pngSmall} alt="board" /> -->
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
            {formatAddress(encodeHashToBase64(board.boardWithMetadata.creator))}
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
      disabled={mintDisabled}
      class="!bg-primary-500 border-black border-2"
      size="sm">Mint</Button
    >
  </div>
</div>

<Modal bind:open={mintMoveModal}>
  <MintMove bind:open={mintMoveModal} {tokenId} />
</Modal>
