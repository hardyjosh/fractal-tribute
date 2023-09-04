import { bytesToHex, getAddress, hexToBytes } from "viem";
import addresses from "$lib/addresses.json"

export const mintEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.mint)]
export const snapshotEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.snapshot)]
export const claimEvaluable = [getAddress(addresses.interpreter), getAddress(addresses.store), getAddress(addresses.claim)]

export const fetchNftIds = async (): Promise<{ id: Uint8Array, supply: number }[]> => {
    const alchemyKey = import.meta.env.VITE_ALCHEMY_KEY;
    const url = `https://polygon-mumbai.g.alchemy.com/nft/v2/${alchemyKey}/getNFTsForCollection?contractAddress=${addresses.instance}&withMetadata=false&limit=1000`;

    let nfts: { id: Uint8Array, supply: number }[] = []

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
            nfts = data.nfts.map((nft: any) => {
                return { id: hexToBytes(nft.id.tokenId), supply: 0 }
            })
        }
    } catch (error) {
        console.log("There was an error fetching the data", error);
    }
    console.log('before supply', nfts)
    const balanceUrl = `https://polygon-mumbai.g.alchemy.com/nft/v2/${alchemyKey}/getOwnersForCollection?contractAddress=${addresses.instance}&withTokenBalances=true`;

    try {
        const response = await fetch(balanceUrl, {
            method: 'GET',
            headers: {
                'accept': 'application/json'
            }
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const data = await response.json();
        console.log(data)
        if (data && data?.ownerAddresses) {
            data.ownerAddresses.forEach(owner => {
                owner.tokenBalances.forEach(tokenBalance => {
                    if (nfts.find(token => tokenBalance.tokenId == bytesToHex(token.id))) {
                        nfts.find(token => tokenBalance.tokenId == bytesToHex(token.id)).supply += tokenBalance.balance;
                    } else {
                        nfts.push({ id: hexToBytes(tokenBalance.tokenId), supply: tokenBalance.balance })
                    }
                });
            });
        }
    } catch (error) {
        console.log("There was an error fetching the data", error);
    }
    console.log(nfts)
    return nfts;

}