import type { NFT } from "$lib/types";
import { writable } from "svelte/store";
import { bytesToHex, hexToBytes } from "viem";
import addresses from "$lib/addresses.json";

const POLLING_INTERVAL = 60000;

export const fetchNftIds = async (): Promise<NFT[]> => {
    const alchemyKey = import.meta.env.VITE_ALCHEMY_KEY;
    const url = `https://polygon-mumbai.g.alchemy.com/nft/v2/${alchemyKey}/getNFTsForCollection?contractAddress=${addresses.instance}&withMetadata=false&limit=1000`;

    let nfts: NFT[] = []

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
    return nfts;

}

const createNftStore = () => {
    const { subscribe, set } = writable<NFT[]>([]);
    let intervalId: NodeJS.Timeout;
    let subscriberCount = 0;

    const fetch = async () => {
        const ids = await fetchNftIds();
        set(ids);
    };

    const startPolling = () => {
        if (intervalId) return;
        fetch();
        intervalId = setInterval(fetch, POLLING_INTERVAL);
    };

    const stopPolling = () => {
        clearInterval(intervalId);
        intervalId = undefined;
    };

    return {
        fetch,
        subscribe(run, invalidate) {
            subscriberCount += 1;
            if (subscriberCount === 1) {
                startPolling();
            }
            const unsubscribe = subscribe(run, invalidate);

            return () => {
                subscriberCount -= 1;
                unsubscribe();
                if (subscriberCount === 0) {
                    stopPolling();
                }
            };
        },
    };
};

export const nfts = createNftStore();