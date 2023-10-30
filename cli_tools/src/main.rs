use fractal_tribute::board::*;
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
    pixmap.save_png(format!("./pattern-masks/large/{}.png", option + 1)).unwrap();
    // render the small option
    let pixmap = svg_to_png(SvgToPngArgs {
        svg_data: svg_document.clone(),
        scale: 0.15,
    }).unwrap();
    // write the png bytes to a new file
    pixmap.save_png(format!("./pattern-masks/small/{}.png", option + 1)).unwrap();
}

