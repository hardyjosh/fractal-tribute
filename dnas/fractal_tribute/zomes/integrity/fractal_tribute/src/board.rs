use hdi::prelude::*;
use svg::node::Text;
use crate::*;
use svg::node::element::{Rectangle, Circle, Polygon, Use, Definitions};
use svg::Document;

const BOARD_SIZE: usize = 50;

// #[hdk_entry_helper]
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
    pub svg: String,
    // pub png: String,
    pub bytes: Vec<u8>,
    pub creator: AgentPubKey,
    pub creation_hash: ActionHash,
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
                        bytes.extend_from_slice(&[0, 0, 0, 0]);
                    }
                }
            }
        }
        bytes
    }

    pub fn generate_svg(&self) -> String {
        let mut document = Document::new().set("viewBox", (0, 0, BOARD_SIZE * 100, BOARD_SIZE * 100));
        
        let defs = Text::new(include_str!("defs.svg"));

        document = document.add(defs);

        for (x, col) in self.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                let base_x = x as u32 * 100;
                let base_y = y as u32 * 100;
    
                let rect = Rectangle::new()
                    .set("x", base_x)
                    .set("y", base_y)
                    .set("width", 100)
                    .set("height", 100);
                    // .set("fill", "rgb(249, 250, 251)");
    
                // document = document.add(rect);
    
                if let Some(color) = tile.color {
                    let fill = format!("rgb({},{},{})", color.r, color.g, color.b);
                    let rect = rect.set("fill", fill);

                    if let Some(graphic_option) = tile.graphic_option {
                        let mask_attr = format!("url(#m_{})", graphic_option + 1);
                        let rect = rect.set("mask", mask_attr);
                        document = document.add(rect);
                    }
                }
            }
        } 
        document.to_string()
    }

    pub fn generate_svg_data_uri(&self) -> String {
        let svg = self.generate_svg();
        format!("data:image/svg+xml;utf8,{}", svg)
    }

}