import type { Color, GameMove, PixelChange } from "../types";

export const gameMoveToBytes = (gameMove: GameMove): Uint8Array => {
    const numOfChanges = gameMove.changes.length;
    const bytes = new Uint8Array(numOfChanges * 4);

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

const getRandomInt = (max: number): number => {
    return Math.floor(Math.random() * Math.floor(max));
};

const getRandomColor = (): Color => {
    return {
        r: getRandomInt(256),
        g: getRandomInt(256),
        b: getRandomInt(256)
    };
};

export const generateRandomGameMove = (): GameMove => {
    const numOfChanges = getRandomInt(10);
    const changes: PixelChange[] = [];
    for (let i = 0; i < numOfChanges; i++) {
        changes.push({
            x: getRandomInt(BOARD_SIZE),
            y: getRandomInt(BOARD_SIZE),
            color: getRandomColor(),
            graphic_option: getRandomInt(16)  // Assuming 16 graphic options
        });
    }
    return { changes };
};

const BOARD_SIZE = 32; 