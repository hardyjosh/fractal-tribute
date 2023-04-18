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

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const settings = {
    apiKey: import.meta.env.VITE_ALCHEMY_KEY_MUMBAI, // Replace with your Alchemy API Key.
    network: Network.MATIC_MUMBAI, // Replace with your network.
  };

  const alchemy = new Alchemy(settings);

  let fetchNFTsPromise;

  const fetchNFTs = () => {
    fetchNFTsPromise = alchemy.nft.getNftsForContract(
      "0x25001c6b219b88af3d35fdfbd9a0ae7397f9c6f3"
    );
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

  onMount(() => {
    fetchNFTs();
  });
</script>

{#await fetchNFTsPromise}
  Fetching NFTs...
{:then resp}
  {#if resp?.nfts}
    {#each resp.nfts as nft}
      <span>TokenID: {nft.tokenId}</span>
      {#await fetchPayloadsForTokenId(nft.tokenId)}
        Getting details from Holochain...
      {:then details}
        <span>TokenID as hex: {details.tokenIdAsHex}</span>
        <span>Link base: {hexlify(details.linkBase)}</span>
        <span>Link base as base64: {details.linkBaseAsBase64}</span>
        {#if details.decodedContents.length}
          <span>Content from the hApp that has this base:</span>
          {#each details.decodedContents as content}
            {#if content?.err}
              <span>{content.err}</span>
              <span>Raw bytes: {content.payload_bytes}</span>
            {:else}
              <span>Name: {content.name}</span>
              <span>Description: {content.description}</span>
            {/if}
          {/each}
        {:else}
          <span>No content found in the hApp that has this base.</span>
        {/if}
      {/await}
      <!-- {console.log(fetchPayloadsForTokenId(nft.tokenId))} -->
    {/each}
  {/if}
{/await}

<button on:click={fetchNFTs}>Refresh</button>
