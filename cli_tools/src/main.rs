use std::fs::write;
use resvg::tiny_skia::*;
use resvg::usvg::TreeParsing;
use resvg::*;
use fractal_tribute_integrity::board::*;

fn main() {
    // loop for all graphic options
    for option in 0..GRAPHIC_OPTIONS {
        println!("saving pattern mask for option {}", option + 1);
        save_pattern_mask(option as u8);
    }
}

fn save_pattern_mask(option: u8) {
    let svg_document = Board::generate_pattern_mask(option);
    // render the large option
    let pixmap = svg_to_png(SvgToPngArgs {
        svg_data: svg_document.clone(),
        scale: 0.5,
    }).unwrap();
    // write the png bytes to a new file
    // pixmap.save_png(format!("./pattern-masks/large/{}.png", option + 1)).unwrap();
    write_image_buf(pixmap, format!("./pattern-masks/large/{}.cache", option + 1));

    // render the small option
    let pixmap = svg_to_png(SvgToPngArgs {
        svg_data: svg_document.clone(),
        scale: 0.15,
    }).unwrap();
    // write the png bytes to a new file
    // pixmap.save_png(format!("./pattern-masks/small/{}.png", option + 1)).unwrap();
    write_image_buf(pixmap, format!("./pattern-masks/small/{}.cache", option + 1));
}

fn write_image_buf(pixmap: Pixmap, path: String) {
    let png_bytes = pixmap.encode_png().unwrap();
    let image_buf = image::load_from_memory(&png_bytes).unwrap();
    let raw = image_buf.into_rgba8().into_raw();
    let _res = write(path, raw);
}
pub struct SvgToPngArgs {
    pub svg_data: String,
    pub scale: f32,
}

pub fn svg_to_png(args: SvgToPngArgs) -> Result<Pixmap, Box<dyn std::error::Error>> {
    // Parse the SVG string
    let opts = resvg::usvg::Options::default();
    let utree = usvg::Tree::from_str(&args.svg_data, &opts)
        .map_err(|e| format!("Could not parse SVG: {:?} {}", e, args.svg_data))?;
    
    let rtree = resvg::Tree::from_usvg(&utree);

    // Create a pixmap for rendering
    let mut binding = Pixmap::new((rtree.size.width() * args.scale) as u32, (rtree.size.height() * args.scale) as u32)
        .ok_or("Could not create Pixmap")?;
    let mut pixmap = binding.as_mut();
    
    rtree.render(Transform::from_scale(args.scale, args.scale), &mut pixmap);

    let pixmap = pixmap.to_owned();

    Ok(pixmap)
}

