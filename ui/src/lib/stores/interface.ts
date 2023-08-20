import type { Board, EvmKeyBinding, GameMove, GameMoveWithActionHash } from '$lib/types';
import type { AppAgentClient, Record, AgentPubKeyB64, EntryHash, ActionHash, Action, encodeHashToBase64 } from '@holochain/client';
import { writable } from 'svelte/store';
import { type Address, getAddress, bytesToHex } from 'viem'
import { decode, encode } from "@msgpack/msgpack";
import { gameMoveToBytes } from '$lib/game-move';
import { parseBoardBytes } from '$lib/board';

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
        return await this.client.callZome({
            cap_secret: null,
            role_name,
            zome_name,
            fn_name: 'create_evm_key_binding',
            payload: evmKeyBindingEntry,
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
        try {
            return await this.client.callZome({
                cap_secret: null,
                role_name,
                zome_name,
                fn_name: 'create_game_move',
                payload: Array.from(gameMoveToBytes(gameMove)),
            }) as Record
        } catch (e) {
            console.log(e)
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

    // async getSnapshotAt(bucketIndex: number): Promise<Snapshot | undefined> {
    //     console.log("getSnapshotAt called at bucketIndex: " + bucketIndex);
    //     return this.client.callZome({
    //         cap_secret: null,
    //         role_name: 'place_nft',
    //         zome_name: 'place',
    //         fn_name: 'get_snapshot_at',
    //         payload: bucketIndex,
    //     });
    // }
}