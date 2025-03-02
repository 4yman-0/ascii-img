//! ASCII renderer module
#[allow(dead_code)]
use super::{Renderer, common::*};
use image::DynamicImage;

/// Renders the image as Unicode art
pub fn render(options: &Renderer, image: &DynamicImage) -> String {
    let image = process_options(options, image).to_luma8();

    let mut string = string_from_image_dimensions(image.width(), image.height());
    let coeff = u8::MAX as f32 / (options.characters.len() - 1) as f32;
    for (_, line) in image.enumerate_rows() {
        for (_, _, pixel) in line {
            let luminance = pixel[0];
            let character = options.characters[(luminance as f32 / coeff) as usize];
            string.push(character)
        }
        string.push('\n');
    }
    string
}
