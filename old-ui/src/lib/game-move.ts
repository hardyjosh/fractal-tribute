import type { GameMove } from "./types";

export const gameMoveToBytes = (gameMove: GameMove): Uint8Array => {
    const bytes = new Uint8Array(40);

    gameMove.changes.forEach((change, i) => {
        const start = i * 4;

        // Encoding x and graphic option
        bytes[start] = (change.x & 0b11111) | (change.graphic_option << 5);

        // Encoding y and a part of red
        bytes[start + 1] = ((change.y & 0b11111) << 3) | (change.color.r >> 5);

        // Continuing with red and then encoding green
        bytes[start + 2] = ((change.color.r & 0b00000111) << 3) | (change.color.g >> 5);

        // Continuing with green and then encoding blue
        bytes[start + 3] = ((change.color.g & 0b00000111) << 3) | (change.color.b >> 5);
    });

    return bytes;
};