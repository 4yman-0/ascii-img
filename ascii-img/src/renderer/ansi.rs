//! ASCII renderer module
#[allow(dead_code)]
use super::{Renderer, common::*};
use ansi_term::Colour;
use image::{DynamicImage, Pixel, Rgb};

fn normalize_luminance(pixel: &Rgb<u8>) -> Rgb<u8> {
    use std::cmp::max;
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    let max_channel = max(max(r, g), b);
    if max_channel == 0 {
        return Rgb([0, 0, 0]);
    }
    let coeff = u8::MAX as f32 / max_channel as f32;
    Rgb([
        (r as f32 * coeff) as u8,
        (g as f32 * coeff) as u8,
        (b as f32 * coeff) as u8,
    ])
}

/// Renders the image as ANSI art
pub fn render(options: &Renderer, image: &DynamicImage) -> String {
    let image = process_options(options, image).to_rgb8();

    let mut string = string_from_size(image.width(), image.height());
    let coeff = u8::MAX as f32 / (options.characters.len() - 1) as f32;
    let mut last_pixel: Option<Rgb<u8>> = None;

    for line in image.rows() {
        for pixel in line {
            if last_pixel != Some(*pixel) {
                let normalized_pixel = normalize_luminance(pixel);
                string.push_str(
                    &Colour::RGB(
                        normalized_pixel[0],
                        normalized_pixel[1],
                        normalized_pixel[2],
                    )
                    .prefix()
                    .to_string(),
                )
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalize_luminance_test() {
        let pixel = normalize_luminance(&Rgb([255, 255, 255]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 255, 255));

        let pixel = normalize_luminance(&Rgb([255, 0, 0]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 0, 0));

        let pixel = normalize_luminance(&Rgb([100, 100, 100]));
        assert_eq!(pixel[0], 255);
        assert_eq!(pixel[1], 255);
        assert_eq!(pixel[2], 255);
    }
}
