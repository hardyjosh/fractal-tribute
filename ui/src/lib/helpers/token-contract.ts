import { getAddress, hexToBytes } from "viem";
import addresses from "$lib/addresses.json"

export const mintEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.mint)]
export const snapshotEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.snapshot)]
export const claimEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.claim)]

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