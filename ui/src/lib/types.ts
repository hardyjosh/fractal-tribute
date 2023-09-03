import type { ActionHash, AgentPubKey } from '@holochain/client'
import type { Hex } from 'viem';

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
  changed?: boolean
};

export type Board = Tile[][];

export type IncomingBoardWithMetadata = {
  bytes: Uint8Array,
  creator: AgentPubKey
  creationHash: ActionHash
}

export type IncomingBoardWithMetadataAndId = {
  board: IncomingBoardWithMetadata,
  id: Uint8Array,
}

export type BoardWithMetadata = {
  board: Board,
  creator: AgentPubKey,
  creationHash: ActionHash
}

export type BoardWithMetadataAndId = {
  boardWithMetadata: BoardWithMetadata,
  id: Hex
}

// Participation proof
export type AgentParticipation = {
  agent: AgentPubKey;
  evm_key: Uint8Array;
  pixels_changed: number;
  percentage_of_total_pixels_changed: number;
  rank: number;
  message_bytes: Uint8Array;
  signature_bytes: Uint8Array;
}

export type ParticipationProof = {
  total_pixels_changed: number;
  agent_participations: AgentParticipation[];
}