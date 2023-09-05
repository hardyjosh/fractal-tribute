use hdi::prelude::*;
use crate::*;
use svg::node::element::{Rectangle, Circle, Polygon};
use svg::Document;

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
    pub svg: String,
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
        let mut document = Document::new().set("viewBox", (0, 0, 3200, 3200));

        for (x, col) in self.tiles.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                let base_x = x as u32 * 100;
                let base_y = y as u32 * 100;

                // Base rectangle
                let rect = Rectangle::new()
                    .set("x", base_x + 1)
                    .set("y", base_y + 1)
                    .set("width", 98)
                    .set("height", 98)
                    .set("fill", "rgb(249, 250, 251)");

                document = document.add(rect);

                if let Some(color) = tile.color {
                    let fill = format!("rgb({},{},{})", color.r, color.g, color.b);
                    
                    match tile.graphic_option {
                        Some(0) => { // Square
                            let square = Rectangle::new()
                                .set("x", base_x)
                                .set("y", base_y)
                                .set("width", 100)
                                .set("height", 100)
                                .set("fill", fill);
                            document = document.add(square);
                        },
                        Some(1) => { // LeftRectangle
                            let rect = Rectangle::new()
                                .set("x", base_x)
                                .set("y", base_y)
                                .set("width", 50)
                                .set("height", 100)
                                .set("fill", fill);
                            document = document.add(rect);
                        },
                        Some(2) => { // UpperRectangle
                            let rect = Rectangle::new()
                                .set("x", base_x)
                                .set("y", base_y)
                                .set("width", 100)
                                .set("height", 50)
                                .set("fill", fill);
                            document = document.add(rect);
                        },
                        Some(3) => { // RightRectangle
                            let rect = Rectangle::new()
                                .set("x", base_x + 50)
                                .set("y", base_y)
                                .set("width", 50)
                                .set("height", 100)
                                .set("fill", fill);
                            document = document.add(rect);
                        },
                        Some(4) => { // LowerRectangle
                            let rect = Rectangle::new()
                                .set("x", base_x)
                                .set("y", base_y + 50)
                                .set("width", 100)
                                .set("height", 50)
                                .set("fill", fill);
                            document = document.add(rect);
                        },
                        Some(5) => { // UpperLeftTriangle
                            let triangle = Polygon::new()
                                .set("points", format!("{},{}, {},{}, {},{}", base_x, base_y, base_x + 100, base_y, base_x, base_y + 100))
                                .set("fill", fill);
                            document = document.add(triangle);
                        },
                        Some(6) => { // UpperRightTriangle
                            let triangle = Polygon::new()
                                .set("points", format!("{},{}, {},{}, {},{}", base_x, base_y, base_x + 100, base_y, base_x + 100, base_y + 100))
                                .set("fill", fill);
                            document = document.add(triangle);
                        },
                        Some(7) => { // LowerRightTriangle
                            let triangle = Polygon::new()
                                .set("points", format!("{},{}, {},{}, {},{}", base_x, base_y + 100, base_x + 100, base_y + 100, base_x + 100, base_y))
                                .set("fill", fill);
                            document = document.add(triangle);
                        },
                        Some(8) => { // LowerLeftTriangle
                            let triangle = Polygon::new()
                                .set("points", format!("{},{}, {},{}, {},{}", base_x, base_y, base_x, base_y + 100, base_x + 100, base_y + 100))
                                .set("fill", fill);
                            document = document.add(triangle);
                        },
                        Some(9) => { // Circle
                            let circle = Circle::new()
                                .set("cx", base_x + 50)
                                .set("cy", base_y + 50)
                                .set("r", 50)
                                .set("fill", fill);
                            document = document.add(circle);
                        },
                        Some(10) => { // Diamond
                            let diamond = Polygon::new()
                                .set("points", format!("{},{}, {},{}, {},{}, {},{}", base_x, base_y + 50, base_x + 50, base_y, base_x + 100, base_y + 50, base_x + 50, base_y + 100))
                                .set("fill", fill);
                            document = document.add(diamond);
                        },
                        _ => {}
                    }
                }
            }
        }

        let svg_str: String = document.to_string();
        format!("data:image/svg+xml;utf8,{}", svg_str)
    }
}