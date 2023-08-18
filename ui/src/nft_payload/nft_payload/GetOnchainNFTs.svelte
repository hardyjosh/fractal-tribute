<script lang="ts">
  import { encodeHashToBase64, type AppAgentClient } from "@holochain/client";
  import { decode } from "@msgpack/msgpack";
  // Github: https://github.com/alchemyplatform/alchemy-sdk-js
  // Setup: npm install alchemy-sdk
  import { Network, Alchemy, BigNumber } from "alchemy-sdk";
  import { ethers } from "ethers";
  import { concat, hexlify } from "ethers/lib/utils";
  import { getContext, onMount } from "svelte";
  import { clientContext } from "../../contexts";
  import type { Payload } from "./types";
  import { Toggle, Spinner } from "flowbite-svelte";
  import { fade, fly } from "svelte/transition";
  import { ArrowPath } from "svelte-heros-v2";
  import "@holochain-open-dev/file-storage/dist/elements/show-image.js";
  import type { FileStorageClient } from "@holochain-open-dev/file-storage/dist/file-storage-client";

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  let fileStorageClient: FileStorageClient = (
    getContext(clientContext) as any
  ).getFileStorageClient();

  const settings = {
    apiKey: import.meta.env.VITE_ALCHEMY_KEY_MUMBAI, // Replace with your Alchemy API Key.
    network: Network.MATIC_MUMBAI, // Replace with your network.
  };

  const alchemy = new Alchemy(settings);

  const collectionAddress = "0x25001c6b219b88af3d35fdfbd9a0ae7397f9c6f3";
  let fetchNFTsPromise;

  let payloadsOnly = true,
    refresh = 1;

  $: if (payloadsOnly || refresh) fetchNFTsPromise = fetchPayloadsForNFTs();

  const fetchPayloadsForNFTs = async () => {
    const resp = await alchemy.nft.getNftsForContract(collectionAddress);

    if (!resp?.nfts?.length) {
      return [];
    }

    const nfts = await Promise.all(
      resp.nfts.map(async (nft) => {
        const payloads = await fetchPayloadsForTokenId(nft.tokenId);
        return {
          ...nft,
          ...payloads,
        };
      })
    );

    if (payloadsOnly) {
      return nfts.filter((nft) => nft.decodedContents.length > 0);
    }
    return nfts;
  };

  const fetchPayloadsForTokenId = async (tokenId: string) => {
    const tokenIdAsHex = BigNumber.from(tokenId).toHexString();
    const linkBase = concat([
      "0x842f24",
      tokenIdAsHex,
      Uint8Array.from([0, 0, 0, 0]),
    ]);
    const linkBaseAsBase64 = encodeHashToBase64(linkBase);
    try {
      const records = await client.callZome({
        cap_secret: null,
        role_name: "nft_payload",
        zome_name: "nft_payload",
        fn_name: "get_payload_from_link",
        payload: linkBase,
      });
      if (!records?.length) {
        return {
          contents: [],
          decodedContents: [],
          tokenIdAsHex,
          linkBase,
          linkBaseAsBase64,
        };
      } else {
        const contents = records.map((record) => {
          return decode((record.entry as any).Present.entry) as Payload;
        });
        const decodedContents = contents.map((content) => {
          try {
            const decoded = decode(content.payload_bytes);
            if (decoded?.fileHash instanceof Uint8Array) {
              return {
                ...decoded,
                fileHash: encodeHashToBase64(decoded.fileHash),
              };
            } else {
              throw Error("Not a Uint8Array");
            }
          } catch (err) {
            // console.log(err);
            return {
              err: "Couldn't decode",
              payload_bytes: content.payload_bytes,
            };
          }
        });
        return {
          contents,
          decodedContents,
          tokenIdAsHex,
          linkBase,
          linkBaseAsBase64,
        };
      }
    } catch (e) {
      console.log(e);
      // error = e;
    }
  };
</script>

<div class="flex justify-between items-center">
  <div>
    <span class="text-white"
      >Collection adddress: <a
        target="_blank"
        rel="noopener noreferrer"
        class="underline"
        href={`https://mumbai.polygonscan.com/address/${collectionAddress}`}
        >{collectionAddress}</a
      ></span
    >
  </div>
  <div class="flex flex-row gap-x-4">
    <div class="flex flex-row gap-x-2 items-center">
      <span>
        <span class="text-white">Show tokens with payloads only</span>
      </span>
      <Toggle bind:checked={payloadsOnly} />
    </div>
    <button
      on:click={() => {
        refresh++;
      }}><ArrowPath class="text-white" /></button
    >
  </div>
</div>

{#await fetchNFTsPromise}
  <div class="flex flex-col min-h-screen w-full items-center mt-48 gap-y-4">
    <Spinner color="gray" />
    <span class="text-white text-2xl font-bold">
      Getting all NFTs in this collection...
    </span>
  </div>
{:then nfts}
  {#if nfts.length}
    <div in:fade={{ duration: 150 }} class="grid grid-cols-4 w-full gap-4 mt-4">
      {#each nfts as nft}
        <div
          class="bg-gray-100 rounded-xl p-4 overflow-hidden flex flex-col text-lg gap-y-2 justify-between"
        >
          {#if nft.decodedContents.length}
            <!-- <span>Content from the hApp that has this base:</span> -->
            {#each nft.decodedContents as content}
              {#if content?.err}
                <span>{content.err}</span>
                <span>Raw bytes: {content.payload_bytes}</span>
              {:else}
              <div>
                <div class="rounded-lg overflow-hidden aspect-square relative">
                  <file-storage-context
                    class="w-full object-cover"
                    client={fileStorageClient}
                  >
                    <show-image image-hash={content.fileHash} class="object-cover absolute inset-0" />
                  </file-storage-context>
                </div>
                <div class="flex flex-col mt-4">
                  <span class="uppercase text-xs font-semibold text-gray-400">Name</span>
                  <span class="">{content.name}</span>
                  <span class="uppercase text-xs font-semibold mt-2 text-gray-400"
                    >Description</span
                  >
                  <span class="">{content.description}</span>
                </div>
                </div>
              {/if}
            {/each}
          {:else}
            <span style="color: red;"
              >No content found in the hApp that has this base.</span
            >
          {/if}
          <div class="text-xs flex flex-col gap-y-2">
            <span class="break-words">TokenID: {nft.tokenId}</span>
            <!-- <span>TokenID as hex: {nft.tokenIdAsHex}</span> -->
            <!-- <span>Link base: {hexlify(details.linkBase)}</span> -->
            <span class="break-words"
              >Link base derived from tokenId: {nft.linkBaseAsBase64}</span
            >
          </div>
        </div>
        <!-- {console.log(fetchPayloadsForTokenId(nft.tokenId))} -->
      {/each}
    </div>
  {:else}
    <div class="flex flex-col min-h-screen w-full items-center mt-48 gap-y-4">
      <span class="text-white text-2xl font-bold"> No NFTs found 😞 </span>
    </div>
  {/if}
{/await}

<style lang="postcss">
  show-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>
