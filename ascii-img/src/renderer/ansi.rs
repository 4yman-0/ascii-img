//! ASCII renderer module
#[allow(dead_code)]
use super::{Renderer, common::*};
use alloc::string::{String, ToString};
use ansi_term::Colour;
use image::{DynamicImage, Rgb};

/// Renders the image as ANSI art
pub fn render(options: &Renderer, image: &DynamicImage) -> String {
    let image = process_options(options, image).into_rgb8();

    let mut string = string_from_size(image.width(), image.height());
    let characters = options.characters.get();
    let coeff = u8::MAX as f32 / (characters.len() - 1) as f32;
    let mut last_pixel: Option<Rgb<u8>> = None;

    for line in image.rows() {
        for pixel in line {
            if last_pixel != Some(*pixel) {
                let normalized_pixel = saturate(pixel);
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
            let luminance = linear_luma_from_rgb(pixel);
            let character = characters[(luminance as f32 / coeff).round() as usize];
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
    fn saturate_test() {
        let pixel = saturate(&Rgb([255, 255, 255]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 255, 255));

        let pixel = saturate(&Rgb([255, 0, 0]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 0, 0));

        let pixel = saturate(&Rgb([100, 100, 100]));
        assert_eq!(pixel[0], 255);
        assert_eq!(pixel[1], 255);
        assert_eq!(pixel[2], 255);
    }
}
