//! ASCII 256 colors renderer module
use super::{Renderer, common::*};
use alloc::string::{String, ToString};
use ansi_term::Colour;
use ansi_colours::ColourExt;
use image::Rgb;

pub struct Ansi256Renderer;

impl Renderer for Ansi256Renderer {
	#[allow(dead_code)]
    fn render_pixel(&self, pixel: &Rgb<u8>, characters: &[char], coeff: f32) -> (String, char) {
        let normalized_pixel = saturate(pixel);
        let color_code = Colour::approx_rgb(
            normalized_pixel[0],
            normalized_pixel[1],
            normalized_pixel[2],
        ).prefix().to_string();
        
        let luminance = linear_luma_from_rgb(pixel).unwrap();
        let character = characters[(luminance as f32 / coeff).round() as usize];
        
        (color_code, character)
    }
}
