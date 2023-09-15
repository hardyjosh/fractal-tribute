import type { DnaProperties, TransformedDnaProperties } from "$lib/types";
import { getAddress } from "viem";

export async function transformDnaProperties(original: DnaProperties): Promise<TransformedDnaProperties> {
    return {
        nftContractAddress: getAddress(original.nft_contract_address),
        paymentTokenAddress: getAddress(original.payment_token_address),
        gameEndTime: new Date(parseInt(original.game_end_time) * 1000),
        gameMasterEvmKey: getAddress(original.game_master_evm_key),
        chainId: parseInt(original.chain_id)
    };
}