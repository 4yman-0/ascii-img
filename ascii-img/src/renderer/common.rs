//! Renderer common behavior module
//! Contains function for use in other renderers

use crate::Renderer;
use image::DynamicImage;

/// Font aspect ratio
/// Asume aspect ration for monospace fonts since fonts are rarely square
const FONT_ASPECT_RATIO: f32 = 1. / 2.;

/// Resizes an image with the `renderer`'s `width` and `height`
fn resize(options: &Renderer, image: &DynamicImage) -> DynamicImage {
    match (options.width, options.height) {
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

/// Apply common transformation to an image usinng the renderer options
pub fn process_options(options: &Renderer, image: &DynamicImage) -> DynamicImage {
    let mut image = resize(options, image);
    if options.invert {
        image.invert();
    };

    image
}

/// Creates an empty string to store results for renderers
pub fn string_from_size(width: u32, height: u32) -> String {
    String::with_capacity((width * height) as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resize_test() {
        let image = DynamicImage::new_luma8(100, 100);

        let resized_image = resize(
            &Renderer::default().width(Some(150)).height(Some(150)),
            &image,
        );

        assert_eq!(resized_image.width(), 150);
        assert_eq!(resized_image.height(), 150);
    }

    #[test]
    fn process_options_test() {
        let image = DynamicImage::new_luma8(100, 100);
        process_options(&Renderer::default(), &image);
    }

    #[test]
    fn string_from_size_test() {
        let string = string_from_size(20, 20);
        assert_eq!(string.capacity(), 400);
    }
}
