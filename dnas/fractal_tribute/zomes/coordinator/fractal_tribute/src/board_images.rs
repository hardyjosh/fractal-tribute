use hdk::prelude::*;
use fractal_tribute_integrity::*;
use image::{ImageBuffer, Rgba};
use image::jpeg::JpegEncoder;
use image::png::PngEncoder;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Once;
use std::str::FromStr;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::sync::Mutex;

struct RenderCache {
    map: HashMap<u64, String>,  // hash of BoardToPngInput to rendered PNG data URI
    order: VecDeque<u64>,       // to keep track of order for LRU eviction
    capacity: usize,
}

impl RenderCache {
    fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn get(&mut self, key: &BoardToPngInput) -> Option<String> {
        let hash = self.hash_key(key);
        if let Some(val) = self.map.get(&hash) {
            // Move the accessed key to the end for LRU
            self.order.retain(|&k| k != hash);
            self.order.push_back(hash);
            Some(val.clone())
        } else {
            None
        }
    }

    fn insert(&mut self, key: BoardToPngInput, value: String) {
        let hash = self.hash_key(&key);
        if self.map.len() >= self.capacity {
            if let Some(least_used) = self.order.pop_front() {
                self.map.remove(&least_used);
            }
        }
        self.order.push_back(hash);
        self.map.insert(hash, value);
    }

    fn hash_key(&self, key: &BoardToPngInput) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.board.hash(&mut hasher);
        key.board_size.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive (Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

const SMALL_MASKS: [&'static [u8]; GRAPHIC_OPTIONS] = [
    include_bytes!("../../../../../../pattern-masks/small/1.png"),
    include_bytes!("../../../../../../pattern-masks/small/2.png"),
    include_bytes!("../../../../../../pattern-masks/small/3.png"),
    include_bytes!("../../../../../../pattern-masks/small/4.png"),
    include_bytes!("../../../../../../pattern-masks/small/5.png"),
    include_bytes!("../../../../../../pattern-masks/small/6.png"),
    include_bytes!("../../../../../../pattern-masks/small/7.png"),
    include_bytes!("../../../../../../pattern-masks/small/8.png"),
    include_bytes!("../../../../../../pattern-masks/small/9.png"),
    include_bytes!("../../../../../../pattern-masks/small/10.png"),
    include_bytes!("../../../../../../pattern-masks/small/11.png"),
    include_bytes!("../../../../../../pattern-masks/small/12.png"),
    include_bytes!("../../../../../../pattern-masks/small/13.png"),
    include_bytes!("../../../../../../pattern-masks/small/14.png"),
    include_bytes!("../../../../../../pattern-masks/small/15.png"),
    include_bytes!("../../../../../../pattern-masks/small/16.png"),
    include_bytes!("../../../../../../pattern-masks/small/17.png"),
];

const LARGE_MASKS: [&'static [u8]; GRAPHIC_OPTIONS] = [
    include_bytes!("../../../../../../pattern-masks/large/1.png"),
    include_bytes!("../../../../../../pattern-masks/large/2.png"),
    include_bytes!("../../../../../../pattern-masks/large/3.png"),
    include_bytes!("../../../../../../pattern-masks/large/4.png"),
    include_bytes!("../../../../../../pattern-masks/large/5.png"),
    include_bytes!("../../../../../../pattern-masks/large/6.png"),
    include_bytes!("../../../../../../pattern-masks/large/7.png"),
    include_bytes!("../../../../../../pattern-masks/large/8.png"),
    include_bytes!("../../../../../../pattern-masks/large/9.png"),
    include_bytes!("../../../../../../pattern-masks/large/10.png"),
    include_bytes!("../../../../../../pattern-masks/large/11.png"),
    include_bytes!("../../../../../../pattern-masks/large/12.png"),
    include_bytes!("../../../../../../pattern-masks/large/13.png"),
    include_bytes!("../../../../../../pattern-masks/large/14.png"),
    include_bytes!("../../../../../../pattern-masks/large/15.png"),
    include_bytes!("../../../../../../pattern-masks/large/16.png"),
    include_bytes!("../../../../../../pattern-masks/large/17.png"),
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

static INIT_CACHE: Once = Once::new();
static mut RENDER_CACHE: Option<Mutex<RenderCache>> = None;

fn get_render_cache() -> &'static Mutex<RenderCache> {
    unsafe {
        INIT_CACHE.call_once(|| {
            RENDER_CACHE = Some(Mutex::new(RenderCache::new(100))); // cache capacity of 100
        });
        RENDER_CACHE.as_ref().unwrap()
    }
}

static INIT_LOCK: Mutex<()> = Mutex::new(());
static mut LARGE_MASK_IMAGES: Option<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> = None;
static mut SMALL_MASK_IMAGES: Option<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>> = None;

fn initialize_masks() {
    // First check (outside the lock)
    unsafe {
        if LARGE_MASK_IMAGES.is_some() && SMALL_MASK_IMAGES.is_some() {
            return;
        }
    }
    
    // Acquire the lock
    let _guard = INIT_LOCK.lock().unwrap();
    
    // Second check (inside the lock)
    unsafe {
        if LARGE_MASK_IMAGES.is_some() && SMALL_MASK_IMAGES.is_some() {
            return;
        }
        
        let mut small_images = Vec::with_capacity(GRAPHIC_OPTIONS);
        for &mask_data in SMALL_MASKS.iter() {
            small_images.push(image::load_from_memory(mask_data).unwrap().to_rgba8());
        }
        SMALL_MASK_IMAGES = Some(small_images);

        let mut large_images = Vec::with_capacity(GRAPHIC_OPTIONS);
        for &mask_data in LARGE_MASKS.iter() {
            large_images.push(image::load_from_memory(mask_data).unwrap().to_rgba8());
        }
        LARGE_MASK_IMAGES = Some(large_images);
    }
}

#[hdk_entry_helper]
#[derive(PartialEq)]
pub enum BoardSize {
    Small = 600,
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
#[derive(Hash, Clone)]
pub struct BoardToPngInput {
    board: BoardInput,
    board_size: String,
}

#[hdk_extern]
pub fn board_to_png(input: BoardToPngInput) -> ExternResult<String> {
    let input_for_cache = input.clone();
    
    // First, get the cache
    let mut cache = get_render_cache().lock().unwrap();

    // Check if image is in cache
    if let Some(data_uri) = cache.get(&input) {
        return Ok(data_uri);
    }

    // Try to get the result from the cache first.
    if let Some(cached_result) = cache.get(&input) {
        return Ok(cached_result);
    }

    let board_size = input.board_size.parse::<BoardSize>().map_err(|_| {
        wasm_error!("Invalid board size provided")
    })?;
    let board = Board::from_board_input(input.board).map_err(|e| wasm_error!(e))?;

    // Ensure masks are initialized
    initialize_masks();

    // Use the preloaded images
    let mask_images = if board_size == BoardSize::Small {
        unsafe { SMALL_MASK_IMAGES.as_ref().unwrap() }
    } else {
        unsafe { LARGE_MASK_IMAGES.as_ref().unwrap() }
    };
    
    let tile_size = board_size as u32 / BOARD_SIZE as u32;
 
    let img_buffer = draw_board(board, &mask_images[..], tile_size as u32);

    let mut buffer = Cursor::new(Vec::new());
    let mut encoder = PngEncoder::new(&mut buffer);
    encoder.encode(&img_buffer, img_buffer.width(), img_buffer.height(), image::ColorType::Rgba8).unwrap();
    
    // base64 encode the buffer into a datauri for bmp
    let bytes = buffer.into_inner();
    let data_uri = format!("data:image/png;base64,{}", base64::encode(bytes));

    // Insert the result into the cache before returning.
    cache.insert(input_for_cache, data_uri.clone());

    Ok(data_uri)
}

fn draw_board(board: Board, mask_images: &[ImageBuffer<Rgba<u8>, Vec<u8>>], tile_size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut canvas = ImageBuffer::new(BOARD_SIZE as u32 * tile_size, BOARD_SIZE as u32 * tile_size);
    // Fill the canvas with white
    canvas.pixels_mut().for_each(|p| *p = Rgba([255, 255, 255, 255]));

    for (x, row) in board.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
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
                                        let fill = Rgba([color.r, color.g, color.b, mask_pixel[3]]);
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
                                        let fill = Rgba([color.r, color.g, color.b, 255]);
                                        
                                        // Blend the mask pixel with the fill color
                                        let blended_r = (mask_pixel[0] as f32 * mask_pixel[3] as f32 / 255.0 + fill[0] as f32 * (1.0 - mask_pixel[3] as f32 / 255.0)) as u8;
                                        let blended_g = (mask_pixel[1] as f32 * mask_pixel[3] as f32 / 255.0 + fill[1] as f32 * (1.0 - mask_pixel[3] as f32 / 255.0)) as u8;
                                        let blended_b = (mask_pixel[2] as f32 * mask_pixel[3] as f32 / 255.0 + fill[2] as f32 * (1.0 - mask_pixel[3] as f32 / 255.0)) as u8;
                                        let blended_pixel = Rgba([blended_r, blended_g, blended_b, 255]);
                                        
                                        canvas.put_pixel(px, py, blended_pixel);
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