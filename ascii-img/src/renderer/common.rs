//! Renderer common behavior module
//! Contains function for use in other renderers

use core::{convert::TryInto,};
use image::{DynamicImage, Rgb};

/// Font aspect ratio
/// Asume aspect ration for monospace fonts since fonts are rarely square
const FONT_ASPECT_RATIO: f32 = 1. / 2.;

/// Return the luminance of an RGB pixel using a simple, linear algorithm
pub fn linear_luma_from_rgb(pixel: &Rgb<u8>) -> u8 {
    let sum: u16 = pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16;
    (sum / 3).try_into().unwrap()
}

/// Resizes an image with optional `width` and `height`
pub fn resize(dimensions: &(Option<u32>, Option<u32>), image: &DynamicImage) -> DynamicImage {
    match *dimensions {
        (Some(width), Some(height)) => image.thumbnail_exact(width, height),
        (Some(width), None) => {
            image.thumbnail_exact(width, (width as f32 * FONT_ASPECT_RATIO) as u32)
        }
        (None, Some(height)) => {
            image.thumbnail_exact((height as f32 / FONT_ASPECT_RATIO) as u32, height)
        }
        (None, None) => image.thumbnail_exact(
            image.width(),
            (image.height() as f32 * FONT_ASPECT_RATIO) as u32,
        ),
    }
}

/// Returns a satuated copy of the provided pixel
pub fn saturate(pixel: &Rgb<u8>) -> Rgb<u8> {
    use core::cmp::max;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linear_luma_from_rgb_test() {
        let luma = linear_luma_from_rgb(&Rgb([0, 0, 0]));
        assert_eq!(luma, 0);

        let luma = linear_luma_from_rgb(&Rgb([255, 255, 255]));
        assert_eq!(luma, 255);
    }

    #[test]
    fn resize_test() {
        let image = DynamicImage::new_luma8(100, 100);
        let resized_image = resize(&(Some(150_u32), Some(150_u32)), &image);

        assert_eq!(resized_image.width(), 150);
        assert_eq!(resized_image.height(), 150);
    }

    #[test]
    fn saturate_test() {
        let pixel = saturate(&Rgb([255, 255, 255]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 255, 255));

        let pixel = saturate(&Rgb([255, 0, 0]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 0, 0));

        let pixel = saturate(&Rgb([100, 100, 100]));
        assert_eq!((pixel[0], pixel[1], pixel[2]), (255, 255, 255));
    }
}
