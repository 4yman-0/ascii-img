//! ASCII renderer module
use super::{Renderer, RendererConfig, common::*};
use alloc::string::String;
use image::{DynamicImage, Rgb};

pub struct UnicodeRenderer;


impl Renderer for UnicodeRenderer {
	#[allow(dead_code)]
    fn render_pixel(&self, pixel: &Rgb<u8>, characters: &[char], coeff: f32) -> (String, char) {
        let luminance = linear_luma_from_rgb(pixel);
        let character = characters[(luminance as f32 / coeff).round() as usize];
        (String::new(), character)
    }

    fn preprocess_image(&self, image: &DynamicImage, config: &RendererConfig) -> DynamicImage {
        // Unicode renderer works with grayscale images
        let mut image = resize(&(config.width(), config.height()), image);
        if config.invert {
            image.invert();
        };

        image.to_luma8().into()
    }
}
