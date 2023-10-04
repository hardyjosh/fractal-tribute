import type { AgentParticipation, InputParticipationProof, ParticipationProof } from "$lib/types";
import { bytesToBigint, encodePacked, keccak256, pad, hexToBytes, type Address, parseEther, bytesToHex, createWalletClient, custom, http, getAddress, type Hex } from 'viem';
import addresses from '$lib/addresses.json';
import { paymentTokenAddress } from "$lib/stores";
import { privateKeyToAccount } from "viem/accounts";
import { get } from "svelte/store";
import { walletClient } from "svelte-wagmi-stores";

export const signParticipations = async (participation: ParticipationProof, privateKey?: string): Promise<ParticipationProof> => {
    console.log(participation);
    const proofs = await Promise.all(participation.agent_participations.map(async (p) => {
        const evmBigint = bytesToBigint(pad(p.evm_key, { size: 32 }));
        const tokenBigint = bytesToBigint(pad(hexToBytes(get(paymentTokenAddress)), { size: 32 }));
        const contractBigint = bytesToBigint(pad(hexToBytes(addresses.instance as Address), { size: 32 }));
        const percentageBigint = BigInt(p.pixels_changed) * BigInt(1000000000000000000) / BigInt(participation.total_pixels_changed);
        console.log(p.percentage_of_total_pixels_changed.toString());
        const message = encodePacked(
            ['uint256', 'uint256', 'uint256', 'uint256'],
            [evmBigint, percentageBigint, tokenBigint, contractBigint]
        );
        console.log(hexToBytes(message))
        const hash = keccak256(message);
        if (bytesToHex(p.message_bytes) !== hash) throw Error('message hash is incorrect');
        const wallet = get(walletClient);
        console.log(wallet)
        const signature = await wallet.signMessage({ account: privateKeyToAccount(privateKey as Hex), message: { raw: hash } });
        return {
            ...p,
            signature_bytes: hexToBytes(signature)
        }
    }));
    return { total_pixels_changed: participation.total_pixels_changed, agent_participations: proofs };
}

export const constructSignedContext = (agentParticipation: AgentParticipation, signer: Hex, totalPixelsChanged: number) => {
    const context = [
        bytesToBigint(pad(agentParticipation.evm_key, { size: 32 })),
        BigInt(agentParticipation.pixels_changed) * BigInt(1000000000000000000) / BigInt(totalPixelsChanged),
        bytesToBigint(pad(hexToBytes(get(paymentTokenAddress)), { size: 32 })),
        bytesToBigint(pad(hexToBytes(addresses.instance as Address), { size: 32 })),
    ]
    const signature = bytesToHex(agentParticipation.signature_bytes);
    return {
        context,
        signature,
        signer: getAddress(signer)
    }
}

export const transformParticipationProof = (participation: ParticipationProof): InputParticipationProof => {
    // need to convert all byte arrays to arrays with Array.from
    const agent_participations = participation.agent_participations.map((p) => {
        return {
            ...p,
            message_bytes: Array.from(p.message_bytes),
            signature_bytes: Array.from(p.signature_bytes),
            agent: Array.from(p.agent),
            evm_key: Array.from(p.evm_key)
        }
    })
    return { ...participation, agent_participations }
}