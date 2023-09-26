use hdi::prelude::*;
use svg::node::Text;
use crate::*;
use svg::node::element::{Rectangle, Circle, Polygon, Use, Definitions, Group};
use svg::{Document, Node};

pub const BOARD_SIZE: usize = 40;

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

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String
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

    pub fn generate_svg_document(&self) -> Document {
        let mut document = Document::new()
            .set("viewBox", (0, 0, BOARD_SIZE * 100, BOARD_SIZE * 100));
    
        let mut groups: Vec<Group> = vec![Group::new(); 16];  // Create 16 empty groups
        let mut bg_group = Group::new();
    
        for (x, col) in self.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                let base_x = x as u32 * 100;
                let base_y = y as u32 * 100;
    
                let mut rect = Rectangle::new()
                    .set("x", base_x)
                    .set("y", base_y)
                    .set("width", 100)
                    .set("height", 100);
    
                let mut bg_rect = Rectangle::new()
                    .set("x", base_x)
                    .set("y", base_y)
                    .set("width", 100)
                    .set("height", 100);
    
                if let Some(color) = tile.color {
                    let fill = format!("rgb({},{},{})", color.r, color.g, color.b);
                    if tile.graphic_option < Some(16) {
                        bg_rect = bg_rect.set("fill", "white");
                        rect = rect.set("fill", fill);
    
                    } else {
                        bg_rect = bg_rect.set("fill", fill);
                        rect = rect.set("fill", "white");
                    }
                    if let Some(mut graphic_option) = tile.graphic_option {
                        graphic_option = graphic_option % 16;
                        // Add the rect to the appropriate group based on its mask.
                        groups[graphic_option as usize].append(rect);
                    }
                    // Add the bg_rect to the bg_group
                    bg_group.append(bg_rect);
                }
            }
        }
    
        // Add bg_group to the document
        document = document.add(bg_group);
    
        // Add mask attribute to groups and add them to the document
        for (i, group) in groups.iter_mut().enumerate() {
            let mask_attr = format!("url(#m_{})", i % 16 + 1);
            group.assign("mask", mask_attr);
            document = document.add(group.clone());
        }

        document
    }

    pub fn generate_svg(&self) -> String {
        let document = self.generate_svg_document();
        document.to_string()
    }
    
    pub fn generate_svg_data_uri(&self) -> String {
        let mut document = self.generate_svg_document();
        let defs = Text::new(include_str!("defs.svg"));
        document = document.add(defs);
        let svg_string = document.to_string();

        format!("data:image/svg+xml;base64,{}", base64::encode(svg_string.clone()))
    }

}