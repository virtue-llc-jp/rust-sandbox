mod mandelbrot;
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use std::fs::File;

fn main() {
    println!("Hello, world!");
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);
    encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8).expect("error writing PNG file");
    // Cannot use `?` operator. Why?
    Ok(())
}
