mod mandelbrot;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder, ImageError};
use num::Complex;
use rayon::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );

        std::process::exit(1);
    }

    let bounds = mandelbrot::parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left =
        mandelbrot::parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right =
        mandelbrot::parse_complex(&args[4]).expect("error parsing lower right corner point");

    // Multi thread rendering;
    let pixels = multi_threads_render(bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);
    encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;
    Ok(())
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = mandelbrot::pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match mandelbrot::escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn multi_threads_render(bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Vec<u8> {
    let mut pixels = vec![0; bounds.0 * bounds.1];
    // Scope of slicing up `pixels` into horizontal bands.
    {
        let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(bounds.0).enumerate().collect();

        bands.into_par_iter().for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left =
                mandelbrot::pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                mandelbrot::pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });
    }

    pixels
}
