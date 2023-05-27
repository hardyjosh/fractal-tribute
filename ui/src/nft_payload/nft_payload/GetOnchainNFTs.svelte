<script lang="ts">
  import type { AppAgentClient } from "@holochain/client";
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

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

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
    const linkBaseAsBase64 = btoa(String.fromCharCode.apply(null, linkBase));
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
            return decode(content.payload_bytes);
          } catch (err) {
            console.log(err);
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
          class="bg-gray-100 rounded-xl p-4 overflow-hidden flex flex-col text-lg"
        >
          <span class="break-words">TokenID: {nft.tokenId}</span>
          <!-- <span>TokenID as hex: {details.tokenIdAsHex}</span> -->
          <!-- <span>Link base: {hexlify(details.linkBase)}</span> -->
          <span class="break-words"
            >Link base as base64: u{nft.linkBaseAsBase64}</span
          >
          {#if nft.decodedContents.length}
            <!-- <span>Content from the hApp that has this base:</span> -->
            {#each nft.decodedContents as content}
              {#if content?.err}
                <span>{content.err}</span>
                <span>Raw bytes: {content.payload_bytes}</span>
              {:else}
                <span style="color: green;">Name: {content.name}</span>
                <span style="color:green;"
                  >Description: {content.description}</span
                >
              {/if}
            {/each}
          {:else}
            <span style="color: red;"
              >No content found in the hApp that has this base.</span
            >
          {/if}
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
