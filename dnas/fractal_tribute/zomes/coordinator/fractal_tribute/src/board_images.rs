use hdk::prelude::*;
use fractal_tribute_integrity::*;
use image::{ImageBuffer, Rgba};
use image::bmp::BmpEncoder;
use image::png::PngEncoder;
use image::jpeg::JpegEncoder;
use serde::de;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Once;
use std::str::FromStr;
use image::ImageEncoder;

#[derive (Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

const SMALL_MASKS: [&'static [u8]; GRAPHIC_OPTIONS] = [
    include_bytes!("./patterns/small/1.png"),
    include_bytes!("./patterns/small/2.png"),
    include_bytes!("./patterns/small/3.png"),
    include_bytes!("./patterns/small/4.png"),
    include_bytes!("./patterns/small/5.png"),
    include_bytes!("./patterns/small/6.png"),
    include_bytes!("./patterns/small/7.png"),
    include_bytes!("./patterns/small/8.png"),
    include_bytes!("./patterns/small/9.png"),
    include_bytes!("./patterns/small/10.png"),
    include_bytes!("./patterns/small/11.png"),
    include_bytes!("./patterns/small/12.png"),
    include_bytes!("./patterns/small/13.png"),
    include_bytes!("./patterns/small/14.png"),
    include_bytes!("./patterns/small/15.png"),
    include_bytes!("./patterns/small/16.png"),
    include_bytes!("./patterns/small/17.png"),
];

const LARGE_MASKS: [&'static [u8]; GRAPHIC_OPTIONS] = [
    include_bytes!("./patterns/large/1.png"),
    include_bytes!("./patterns/large/2.png"),
    include_bytes!("./patterns/large/3.png"),
    include_bytes!("./patterns/large/4.png"),
    include_bytes!("./patterns/large/5.png"),
    include_bytes!("./patterns/large/6.png"),
    include_bytes!("./patterns/large/7.png"),
    include_bytes!("./patterns/large/8.png"),
    include_bytes!("./patterns/large/9.png"),
    include_bytes!("./patterns/large/10.png"),
    include_bytes!("./patterns/large/11.png"),
    include_bytes!("./patterns/large/12.png"),
    include_bytes!("./patterns/large/13.png"),
    include_bytes!("./patterns/large/14.png"),
    include_bytes!("./patterns/large/15.png"),
    include_bytes!("./patterns/large/16.png"),
    include_bytes!("./patterns/large/17.png"),
];

trait ImageBufferExt {
    fn clear(&mut self);
}

impl ImageBufferExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn clear(&mut self) {
        for pixel in self.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

static INIT: Once = Once::new();
static mut LARGE_MASK_IMAGES: Option<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> = None;
static mut SMALL_MASK_IMAGES: Option<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> = None;

fn initialize_masks() {
    INIT.call_once(|| {
        debug!("initializing masks");
        let mut small_images = Vec::with_capacity(GRAPHIC_OPTIONS);
        for &mask_data in SMALL_MASKS.iter() {
            debug!("loading small mask");
            small_images.push(image::load_from_memory(mask_data).unwrap().to_rgba8());
        }
        unsafe {
            SMALL_MASK_IMAGES = Some(small_images);
        }

        let mut large_images = Vec::with_capacity(GRAPHIC_OPTIONS);
        for &mask_data in LARGE_MASKS.iter() {
            debug!("loading large mask");
            large_images.push(image::load_from_memory(mask_data).unwrap().to_rgba8());
        }
        debug!("masks loaded");
        unsafe {
            LARGE_MASK_IMAGES = Some(large_images);
        }
        debug!("masks initialized");
    });
}

#[hdk_entry_helper]
#[derive(PartialEq)]
pub enum BoardSize {
    Small = 400,
    Large = 2000,
}

impl FromStr for BoardSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Small" => Ok(BoardSize::Small),
            "Large" => Ok(BoardSize::Large),
            _ => Err(()),
        }
    }
}

#[hdk_entry_helper]
pub struct BoardToPngInput {
    board: BoardInput,
    board_size: String,
}

#[hdk_extern]
pub fn board_to_png(input: BoardToPngInput) -> ExternResult<String> {
    let board_size = input.board_size.parse::<BoardSize>().map_err(|_| {
        wasm_error!("Invalid board size provided")
    })?;
    debug!("board_to_png");
    let board = Board::from_board_input(input.board).map_err(|e| wasm_error!(e))?;
    debug!("reading masks into buffers");

    // Ensure masks are initialized
    initialize_masks();

    // Use the preloaded images
    let mask_images = if board_size == BoardSize::Small {
        unsafe { SMALL_MASK_IMAGES.as_ref().unwrap() }
    } else {
        unsafe { LARGE_MASK_IMAGES.as_ref().unwrap() }
    };
    
    // let mask_images = unsafe { MASK_IMAGES.as_ref().unwrap() };

    let tile_size = board_size as u32 / BOARD_SIZE as u32;
 
    debug!("draw_board");
    let img_buffer = draw_board(board, &mask_images[..], tile_size as u32);

    debug!("encode_bmp");
    let mut buffer = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new(&mut buffer);
    encoder.encode(&img_buffer, img_buffer.width(), img_buffer.height(), image::ColorType::Rgba8).unwrap();
    
    // base64 encode the buffer into a datauri for bmp
    let bytes = buffer.into_inner();
    let data_uri = format!("data:image/jpg;base64,{}", base64::encode(bytes));
    // let bytes = buffer.into_inner();
    debug!("complete");
    Ok(data_uri)
}

fn draw_board(board: Board, mask_images: &[ImageBuffer<Rgba<u8>, Vec<u8>>], tile_size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut canvas = ImageBuffer::new(BOARD_SIZE as u32 * tile_size, BOARD_SIZE as u32 * tile_size);
    // Fill the canvas with white
    canvas.pixels_mut().for_each(|p| *p = Rgba([255, 255, 255, 255]));

    for (x, row) in board.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            debug!("drawing tile at {}, {}", x, y);
            // debug out the tile innfo
            debug!("tile: {:?}", tile);
            if let Some(color) = &tile.color {
                if let Some(graphic_option) = tile.graphic_option {
                    match graphic_option {
                        0..=16 => {
                            let mask = &mask_images[graphic_option as usize];
                            for i in 0..tile_size {
                                for j in 0..tile_size {
                                    let px = x as u32 * tile_size + i;
                                    let py = y as u32 * tile_size + j;
                                    let mask_pixel = mask.get_pixel(px, py);
                                    if mask_pixel[3] != 0 {  // non-alpha
                                        let fill = Rgba([color.r, color.g, color.b, 255]);
                                        canvas.put_pixel(px, py, fill);
                                    }
                                }
                            }
                        },
                        17..=33 => {
                            let mask = &mask_images[graphic_option as usize % 17];
                            for i in 0..tile_size {
                                for j in 0..tile_size {
                                    let px = x as u32 * tile_size + i;
                                    let py = y as u32 * tile_size + j;

                                    let mask_pixel = mask.get_pixel(px, py);
                                    if mask_pixel[3] != 0 {  // non-alpha
                                        canvas.put_pixel(px, py, mask_pixel.clone());
                                    } else {
                                        let fill = Rgba([color.r, color.g, color.b, 255]);
                                        canvas.put_pixel(px, py, fill);
    
                                    }
                                }
                            }
                        },
                        34 => {
                            let fill = Rgba([color.r, color.g, color.b, 255]);
                            for i in 0..tile_size {
                                for j in 0..tile_size {
                                    // debug!("drawing graphic option 35 pixel at {}, {}", x, y);
                                    let px = x as u32 * tile_size + i;
                                    let py = y as u32 * tile_size + j;
                                    canvas.put_pixel(px, py, fill);
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    canvas
}
fn draw_tile(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, pos: Position, fill: Rgba<u8>, tile_size: u32) {
    for i in 0..tile_size {
        for j in 0..tile_size {
            let x = pos.x * tile_size + i;
            let y = pos.y * tile_size + j;
            img.put_pixel(x, y, fill);
        }
    }
}

fn apply_mask(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, mask: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (pixel, mask_pixel) in img.pixels_mut().zip(mask.pixels()) {
        let alpha = pixel[3] as f32 * mask_pixel[3] as f32 / 255.0;
        pixel[3] = alpha.round() as u8;
    }
}

fn apply_overlay_mask(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, mask: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (pixel, mask_pixel) in img.pixels_mut().zip(mask.pixels()) {
        if pixel[3] != 0 {
            pixel[0] = mask_pixel[0];
            pixel[1] = mask_pixel[1];
            pixel[2] = mask_pixel[2];
            pixel[3] = mask_pixel[3];
        }
    }
}