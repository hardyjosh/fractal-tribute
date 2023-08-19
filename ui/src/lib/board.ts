import type { Tile } from "./types";

const BOARD_SIZE = 32;

export const parseBoardBytes = (bytes: Uint8Array): Tile[][] => {
    if (bytes.length !== BOARD_SIZE * BOARD_SIZE * 4) {
        throw new Error("Invalid board bytes length");
    }

    const board: Tile[][] = Array.from({ length: BOARD_SIZE }, () => Array(BOARD_SIZE).fill({}));

    let byteIndex = 0;
    for (let y = 0; y < BOARD_SIZE; y++) {
        for (let x = 0; x < BOARD_SIZE; x++) {
            const r = bytes[byteIndex++];
            const g = bytes[byteIndex++];
            const b = bytes[byteIndex++];
            const graphic_option = bytes[byteIndex++];

            const tile: Tile = {};

            // Only set the color and graphic_option if they're not default (e.g., 0)
            if (r || g || b) {
                tile.color = { r, g, b };
            }

            tile.graphic_option = graphic_option;

            board[y][x] = tile;
        }
    }

    return board;
};