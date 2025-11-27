//! ASCII renderer module
use super::{common::*, Renderer};
use alloc::string::{String, ToString};
use ansi_term::Colour;
use image::Rgb;

pub struct AnsiRenderer;

#[allow(dead_code)]
impl Renderer for AnsiRenderer {
	/// Renders the image as ANSI art
    fn render_pixel(&self, pixel: &Rgb<u8>, characters: &[char], coeff: f32) -> (String, char) {
        let normalized_pixel = saturate(pixel);
        let color_code = Colour::RGB(
            normalized_pixel[0],
            normalized_pixel[1],
            normalized_pixel[2],
        ).prefix().to_string();
        
        let luminance = linear_luma_from_rgb(pixel);
        let character = characters[(luminance as f32 / coeff).round() as usize];
        
        (color_code, character)
    }
}
