import { IFlowERC1155V3 } from "$lib/abi/IFlowERC1155V3";
import type { ActionHash } from "@holochain/client";
import { makeContractStore } from "svelte-wagmi-stores";
import { get } from "svelte/store";
import type { Address } from "viem";
import { getAddress, hexToBytes } from "viem";

const addresses = {
    "expression": "0x0046eAC07579c27185CFBB044721f85174382170",
    "instance": "0x8C7aB420A7a0fd520193302EF7937d3d0b4BAbc4",
    "interpreter": "0x69c0783A613d33E19Ad91aeD51A616a4b27aE3a2",
    "store": "0xeCe6950c86BbD6F7a1A4CEEf6254B7C922dDd4F0"
}
export const evaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.expression)]

export const token = makeContractStore(IFlowERC1155V3, addresses.instance as Address)

export const fetchNftIds = async (): Promise<Uint8Array[]> => {
    const alchemyKey = import.meta.env.VITE_ALCHEMY_KEY;
    const url = `https://polygon-mumbai.g.alchemy.com/nft/v2/${alchemyKey}/getNFTsForCollection?contractAddress=${addresses.instance}&withMetadata=false&limit=1000`;

    try {
        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'accept': 'application/json'
            }
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const data = await response.json();
        if (data && data?.nfts) {
            return data.nfts.map((nft: any) => {
                return hexToBytes(nft.id.tokenId)
            })
        }
    } catch (error) {
        console.error("There was an error fetching the data", error);
    }
}