import type { ParticipationProof } from "$lib/types";
import { bytesToBigint, encodePacked, keccak256, pad, hexToBytes, type Address, parseEther, bytesToHex } from 'viem';
import addresses from '$lib/addresses.json';
import { paymentTokenAddress } from "$lib/stores";

export const signParticipation = async (participation: ParticipationProof, privateKey?: string) => {

    const proofs = participation.agent_participations.map((p) => {
        const evmBigint = bytesToBigint(pad(p.evm_key, { size: 32 }));
        const tokenBigint = bytesToBigint(pad(hexToBytes(paymentTokenAddress), { size: 32 }));
        const contractBigint = bytesToBigint(pad(hexToBytes(addresses.instance as Address), { size: 32 }));
        const percentageBigint = parseEther(p.percentage_of_total_pixels_changed.toString());
        const message = encodePacked(
            ['uint256', 'uint256', 'uint256', 'uint256'],
            [evmBigint, percentageBigint, tokenBigint, contractBigint]
        );
        const hash = keccak256(message);
        console.log('message hash is correct', bytesToHex(p.message_bytes) === hash);

    });
}