//! ASCII renderer module
#[allow(dead_code)]
use super::{Renderer, common::*};
use image::{DynamicImage, Pixel};

/// Writes the image as ASCII art to `string`
pub fn render(options: &Renderer, image: &DynamicImage) -> String {
	let mut image = scale(image, options.width, options.height);
	if options.invert {
		invert(&mut image);
	}
	let image = image.to_luma8();
	
    let mut string = String::with_capacity((image.width() * image.height()) as usize);
    let coeff = u8::MAX as f32 / (options.characters.len() - 1) as f32;
    for (_, line) in image.enumerate_rows() {
        for (_, _, pixel) in line {
            let luminance = *pixel.channels().first().unwrap();
            let character = options.characters[(luminance as f32 / coeff) as usize];
            string.push(character)
        }
        string.push('\n');
    }
    string
}
