//! Renderer common behavior module
//! Contains function for use in other renderers

use super::Renderer;
use image::DynamicImage;

/// Font aspect ratio
/// Asume aspect ration for monospace fonts since fonts are rarely square
const FONT_ASPECT_RATIO: f32 = 1. / 2.;

fn scale(options: &Renderer, image: &DynamicImage) -> DynamicImage {
	match (options.width, options.height) {
	    (Some(width), Some(height)) => image.thumbnail_exact(width, height),
	    (Some(width), None) => image.thumbnail_exact(width, (width as f32 * FONT_ASPECT_RATIO) as u32),
	    (None, Some(height)) => image.thumbnail_exact((height as f32 / FONT_ASPECT_RATIO) as u32, height),
	    (None, None) => image.thumbnail_exact(
	        image.width(),
	        (image.height() as f32 * FONT_ASPECT_RATIO) as u32
	    ),
	}
}

pub fn process_options(options: &Renderer, image: &DynamicImage) -> DynamicImage {
	let mut image = scale(options, image);
	if options.invert {
		image.invert();
	};

	image
}
