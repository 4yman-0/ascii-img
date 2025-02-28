//! Renderer common behavior module
//! Contains function for use in other renderers

use image::DynamicImage;

/// Font aspect ratio
/// Asume 1/2 aspect ration for monospace fonts since fonts are rarely square
const FONT_ASPECT_RATIO: f32 = 1. / 2.;

pub fn scale(image: &DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
	match (width, height) {
	    (Some(width), Some(height)) => image.thumbnail_exact(width, height),
	    (Some(width), None) => image.thumbnail((width as f32 / FONT_ASPECT_RATIO) as u32, u32::MAX),
	    (None, Some(height)) => image.thumbnail(u32::MAX, (height as f32 * FONT_ASPECT_RATIO) as u32),
	    (None, None) => image.resize_exact(
	        image.width(),
	        (image.height() as f32 * FONT_ASPECT_RATIO) as u32,
	        image::imageops::FilterType::Nearest,
	    ),
	}
}

pub fn invert(image: &mut DynamicImage){
	image.invert();
}
