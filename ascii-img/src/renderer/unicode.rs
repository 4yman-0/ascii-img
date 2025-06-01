//! ASCII renderer module
#[allow(dead_code)]
use super::{common::*, Renderer};
use alloc::string::String;
use image::DynamicImage;

/// Renders the image as Unicode art
pub fn render(options: &Renderer, image: &DynamicImage) -> String {
    let image = process_options(options, image).to_luma8();

    let mut string = string_from_size(image.width(), image.height());
    let characters = options.characters().get();
    let coeff = u8::MAX as f32 / (characters.len() - 1) as f32;

    for line in image.rows() {
        for pixel in line {
            let luminance = pixel[0];
            let character = characters[(luminance as f32 / coeff).round() as usize];
            string.push(character)
        }
        string.push('\n');
    }
    string
}
