import type { Color, GameMove, PixelChange } from "../types";

export const gameMoveToBytes = (gameMove: GameMove): Uint8Array => {
    const numOfChanges = gameMove.changes.length;
    const bytes = new Uint8Array(numOfChanges * 5);

    gameMove.changes.forEach((change, i) => {
        const start = i * 5;

        // Encode x and y
        bytes[start] = (change.x & 0b11111) << 3 | (change.y >> 2);
        bytes[start + 1] = ((change.y & 0b00000011) << 6) | (change.graphic_option & 0b1111);

        // Encode the RGB colors
        bytes[start + 2] = change.color.r;
        bytes[start + 3] = change.color.g;
        bytes[start + 4] = change.color.b;
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