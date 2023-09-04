import { concat, bytesToHex, type Hex, keccak256, hexToBytes, pad } from "viem";
import type { Board, BoardWithMetadataAndId, GameMove, IncomingBoardWithMetadataAndId, Tile, IncomingBoardWithMetadata, BoardWithMetadata } from "../types";

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

export const mergeGameMoveIntoBoard = (board: Board, gameMove: GameMove): Board => {

    const newBoard = board.map((row) => row.map((tile) => ({ ...tile })));

    gameMove.changes.forEach((change) => {
        const { x, y, color, graphic_option } = change;
        newBoard[y][x] = { color, graphic_option, changed: true };
    });

    return newBoard;

}

export const tokenIdToLinkBase = (tokenId: Uint8Array): Uint8Array => {
    return concat([
        Uint8Array.from([132, 47, 36]),
        tokenId,
        Uint8Array.from([0, 0, 0, 0]),
    ]);
}

export const actionHashAndAccountToTokenId = (actionHash: Uint8Array, account: Hex): Uint8Array => {
    const accountBytes = pad(hexToBytes(account), { size: 32 });
    const moveHash = hexToBytes(keccak256(actionHash));
    const tokenId = keccak256(concat([accountBytes, moveHash]));
    return hexToBytes(tokenId)
}


export const parseIncomingBoardWithMetadataAndId = (incomingBoardWithMetadataAndId: IncomingBoardWithMetadataAndId): BoardWithMetadataAndId => {
    const { board, id } = incomingBoardWithMetadataAndId;
    const { bytes, creator, creationHash } = board;
    const parsedBoard = parseBoardBytes(bytes);
    return { boardWithMetadata: { board: parsedBoard, creator, creationHash }, id: bytesToHex(id) };
}

export const parseIncomingBoardWithMetadata = (incomingBoardWithMetadata: IncomingBoardWithMetadata): BoardWithMetadata => {
    const { bytes, creator, creationHash } = incomingBoardWithMetadata;
    const parsedBoard = parseBoardBytes(bytes);
    return { board: parsedBoard, creator, creationHash };
}

export enum ShapeOptions {
    Square,
    LeftRectangle,
    UpperRectangle,
    RightRectangle,
    LowerRectangle,
    UpperLeftTriangle,
    UpperRightTriangle,
    LowerRightTriangle,
    LowerLeftTriangle,
    Circle,
    Diamond,
}