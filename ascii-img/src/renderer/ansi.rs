//! ASCII renderer module
#[allow(dead_code)]
use super::{Renderer, common::*};
use image::{DynamicImage, Pixel, Rgb};
use ansi_term::Colour;

fn normalize_luminance(pixel: &Rgb<u8>) -> Rgb<u8> {
	use std::cmp::max;
	let (r, g, b) = (
		pixel[0],
		pixel[1],
		pixel[2],
	);
	let max_channel = max(max(r, g), b);
	if max_channel == 0 {
		return Rgb([0, 0, 0]);
	}
	let coeff = u8::MAX / max_channel;
	Rgb([
		r * coeff,
		g * coeff,
		b * coeff,
	])
}

/// Writes the image as ASCII art to `string`
pub fn render(options: &Renderer, image: &DynamicImage) -> String {
	let mut image = scale(image, options.width, options.height);
	if options.invert {
		invert(&mut image);
	}
	let image = image.to_rgb8();
	
    let mut string = String::with_capacity((image.width() * image.height()) as usize);
    let coeff = u8::MAX as f32 / (options.characters.len() - 1) as f32;
    let mut last_pixel: Option<Rgb<u8>> = None;

    for (_, line) in image.enumerate_rows() {
        for (_, _, pixel) in line {
        	if last_pixel != Some(*pixel) {
        		let normalized_pixel = normalize_luminance(pixel);
        		string.push_str(&Colour::RGB(
        			normalized_pixel[0],
        			normalized_pixel[1],
        			normalized_pixel[2],
        		).prefix().to_string())
        	}
            let luminance = (*pixel).to_luma()[0];
            let character = options.characters[(luminance as f32 / coeff) as usize];
            string.push(character);
 
            last_pixel = Some(*pixel);
        }
        string.push('\n');
    }
    string
}
