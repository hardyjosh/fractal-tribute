import type { Board, EvmKeyBinding, GameMove, GameMoveWithActionHash, ParticipationProof } from '$lib/types';
import type { AppAgentClient, Record, ActionHash } from '@holochain/client';
import { writable } from 'svelte/store';
import { type Address, getAddress, bytesToHex, concat } from 'viem'
import { decode } from "@msgpack/msgpack";
import { gameMoveToBytes, parseBoardBytes } from '$lib/helpers';

export const happ = writable<DnaInterface>()

export const initHapp = (client: AppAgentClient) => {
    happ.set(new DnaInterface(client))
}

const role_name = 'fractal_tribute';
const zome_name = 'fractal_tribute';

export class DnaInterface {

    constructor(client: AppAgentClient) {
        this.client = client;
    }

    private client

    myPubKey(): Uint8Array {
        return this.client.myPubKey;

    }
    // evm key binding
    async createEvmKeyBinding(evmKeyBindingEntry: EvmKeyBinding): Promise<Record> {
        let _evmKeyBinding: any = {}
        _evmKeyBinding.evm_key = Array.from(evmKeyBindingEntry.evm_key)
        _evmKeyBinding.signature_bytes = Array.from(evmKeyBindingEntry.signature_bytes)
        return await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'create_evm_key_binding',
            payload: _evmKeyBinding,
        }) as Record
    }

    async getEvmAddress(): Promise<Address> {
        let addressBytes = await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'get_evm_address',
            payload: null,
        })
        return getAddress(bytesToHex(addressBytes))
    }

    // moves
    async createGameMove(gameMove: GameMove): Promise<Record> {
        console.log('creating move', gameMove)
        try {
            return await this.client.callZome({
                cap_secret: null,
                role_name,
                zome_name,
                fn_name: 'create_game_move',
                payload: Array.from(gameMoveToBytes(gameMove)),
            }) as Record
        } catch (e) {
            console.log(e.data.data)
            throw e
        }
    }

    async getAllMyGameMoves(): Promise<GameMoveWithActionHash[]> {
        const request = await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'get_all_my_game_moves',
            payload: null,
        })
        const records: GameMoveWithActionHash[] = request.map((r: Record) => {
            const gameMove = decode((r.entry as any).Present.entry) as GameMove
            const actionHash = r.signed_action.hashed.hash
            return { gameMove, actionHash }
        })
        return records
    }

    // board
    async getLatestBoard(): Promise<Board> {
        const boardBytes = await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'get_latest_board',
            payload: null,
        })
        return parseBoardBytes(boardBytes)
    }

    async getBoardAtMove(actionHash: ActionHash): Promise<Board> {
        const boardBytes = await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'get_board_at_move',
            payload: actionHash,
        })
        return parseBoardBytes(boardBytes)
    }

    async getBoardFromTokenId(tokenId: Uint8Array): Promise<Board> {
        const linkBase = concat([
            Uint8Array.from([132, 47, 36]),
            tokenId,
            Uint8Array.from([0, 0, 0, 0]),
        ]);
        const boardBytes = await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'get_board_from_link',
            payload: linkBase,
        })
        return parseBoardBytes(boardBytes)
    }

    // participation proof
    async buildAgentParticipation(): Promise<ParticipationProof> {
        const record = await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'build_agent_participation',
            payload: null,
        }) as Record
        return record as any as ParticipationProof
    }
}