import type { AgentParticipation, ParticipationProof } from "$lib/types";
import { bytesToBigint, encodePacked, keccak256, pad, hexToBytes, type Address, parseEther, bytesToHex, createWalletClient, custom, http, getAddress } from 'viem';
import addresses from '$lib/addresses.json';
import { paymentTokenAddress } from "$lib/stores";
import { privateKeyToAccount } from "viem/accounts";
import { get } from "svelte/store";
import { walletClient } from "svelte-wagmi-stores";

export const signParticipations = async (participation: ParticipationProof, privateKey?: string) => {

    const proofs = await Promise.all(participation.agent_participations.map(async (p) => {
        const evmBigint = bytesToBigint(pad(p.evm_key, { size: 32 }));
        const tokenBigint = bytesToBigint(pad(hexToBytes(get(paymentTokenAddress)), { size: 32 }));
        const contractBigint = bytesToBigint(pad(hexToBytes(addresses.instance as Address), { size: 32 }));
        const percentageBigint = parseEther(p.percentage_of_total_pixels_changed.toString());
        const message = encodePacked(
            ['uint256', 'uint256', 'uint256', 'uint256'],
            [evmBigint, percentageBigint, tokenBigint, contractBigint]
        );
        // console.log('message', hexToBytes(message))
        const hash = keccak256(message);
        // console.log('message hash is correct', bytesToHex(p.message_bytes) === hash);
        if (bytesToHex(p.message_bytes) !== hash) throw Error('message hash is incorrect');

        const wallet = get(walletClient);
        const signature = await wallet.signMessage({ account: privateKeyToAccount(import.meta.env.VITE_STEWARD_KEY), message: { raw: hash } });
        return {
            ...p,
            signature_bytes: hexToBytes(signature)
        }
    }));
    // console.log(proofs);
    return { ...participation, agent_participations: proofs };
}

export const constructSignedContext = (agentParticipation: AgentParticipation) => {
    const context = [
        bytesToBigint(pad(agentParticipation.evm_key, { size: 32 })),
        parseEther(agentParticipation.percentage_of_total_pixels_changed.toString()),
        bytesToBigint(pad(hexToBytes(get(paymentTokenAddress)), { size: 32 })),
        bytesToBigint(pad(hexToBytes(addresses.instance as Address), { size: 32 })),
    ]
    const signature = bytesToHex(agentParticipation.signature_bytes);
    const signer = getAddress("0x74423442CEA6B5c90d13C2d7C21B0FcE723ECe6d");
    return {
        context,
        signature,
        signer
    }
}