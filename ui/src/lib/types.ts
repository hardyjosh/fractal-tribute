import type { ActionHash } from '@holochain/client'

// EVM key binding
export type EvmKeyBinding = {
  evm_key: Uint8Array;
  signature_bytes: Uint8Array;
}

// Game moves

export type Color = {
  r: number,
  g: number,
  b: number
};

export type PixelChange = {
  x: number,
  y: number,
  color: Color,
  graphic_option: number
};

export type GameMove = {
  changes: PixelChange[]
};

export type GameMoveWithActionHash = {
  gameMove: GameMove,
  actionHash: ActionHash
}

export type Tile = {
  color?: Color,
  graphic_option?: number
};

export type Board = Tile[][];
