use hdi::prelude::*;
use crate::*;

const BOARD_SIZE: usize = 32;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Board {
    tiles: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq, Copy)]
pub struct Tile {
    color: Option<Color>,
    graphic_option: Option<u8>,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct BoardWithMetadata {
    pub bytes: Vec<u8>,
    pub creator: AgentPubKey,
    pub creationHash: ActionHash,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct BoardWithMetadataAndId {
    pub board: BoardWithMetadata,
    pub id: Vec<u8>,
}

impl Board {
    fn new() -> Self {
        Board {
            tiles: [[Tile { color: None, graphic_option: None }; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn apply_game_move(&mut self, game_move_: &GameMove) {
        for change in &game_move_.changes {
            self.tiles[change.y][change.x].color = Some(change.color);
            self.tiles[change.y][change.x].graphic_option = Some(change.graphic_option);
        }
    }

    pub fn reconstruct_from_game_moves(game_moves: &[GameMove]) -> Self {
        let mut board = Board::new();
        for game_move_ in game_moves {
            board.apply_game_move(game_move_);
        }
        board
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for row in &self.tiles {
            for tile in row {
                match &tile.color {
                    Some(color) => {
                        bytes.push(color.r);
                        bytes.push(color.g);
                        bytes.push(color.b);
                        bytes.push(tile.graphic_option.unwrap_or(0));
                    },
                    None => {
                        // If no color is present, you might want to push a default representation.
                        // For example, push four zero bytes. Adjust as needed.
                        bytes.extend_from_slice(&[0, 0, 0, 0]);
                    }
                }
            }
        }
        bytes
    }
}