import type { ActionHash, AgentPubKey } from '@holochain/client'
import type { Address, Hex } from 'viem';

export type DnaProperties = {
  nft_contract_address: string;
  payment_token_address: string;
  game_end_time: string;
  game_master_evm_key: string;
  chain_id: string;
};

export type TransformedDnaProperties = {
  nftContractAddress: Address;
  paymentTokenAddress: Address;
  gameEndTime: Date;
  gameMasterEvmKey: Address;
  chainId: number;
};

// EVM key binding
export type EvmKeyBinding = {
  evm_key: Uint8Array;
  signature_bytes: Uint8Array;
}

// Profile
export type Profile = {
  name: string;
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

export type BrushTool = 'eye-dropper' | 'eraser' | 'none';

export type Brush = {
  brushTool: BrushTool,
  color: Color,
  graphic_option: number
}

export type Board = Tile[][];

export type IncomingBoardWithMetadata = {
  png: string,
  svg: string,
  bytes: Uint8Array,
  creator: AgentPubKey
  creation_hash: ActionHash
}

export type IncomingBoardWithMetadataAndId = {
  board: IncomingBoardWithMetadata,
  id: Uint8Array,
}

export type BoardWithMetadata = {
  png: string,
  svg: string,
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

// NFTS
export type NFT = { id: Uint8Array, supply: number }