//! ASCII renderer module
#[allow(dead_code)]
use crate::renderer::{RendererOptions, RendererTrait};
use image::{DynamicImage, Pixel, imageops::FilterType};

/// Font aspect ratio
/// Asume 1/2 aspect ration for monospace fonts since fonts are rarely square
const FONT_ASPECT_RATIO: f32 = 1. / 2.;

/// ASCII renderer
/// Renders using ASCII characters only
pub struct AsciiRenderer {
    options: RendererOptions,
    characters: Vec<char>,
}

impl Default for AsciiRenderer {
    fn default() -> Self {
        Self {
        	options: RendererOptions::default(),
        	characters: "@MBHENR#KWXDFPQASUZbdehx8Gm&04LOVYkpq5Tagns69owz$CIu23Jcfry%1v7l+it[]{}?j|()=~!-/<>\"^';,:`.".chars().collect(),
        }
    }
}

#[allow(dead_code)]
impl AsciiRenderer {
    /// Creates a new `AsciiRenderer`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the  width of the renderer image(s)
    pub fn width(mut self, width: Option<u32>) -> Self {
        self.options.width = width;
        self
    }

    /// Sets the height of the renderer image(s)
    pub fn height(mut self, height: Option<u32>) -> Self {
        self.options.height = height;
        self
    }

    /// Sets the characters used in the renderer image(s)
    pub fn characters(mut self, characters: Vec<char>) -> Self {
        self.characters = characters;
        self
    }
}

impl RendererTrait for AsciiRenderer {
    fn render(self, image: &DynamicImage) -> String {
        let width = self.options.width;
        let height = self.options.height;
        let image = match (width, height) {
            (Some(width), Some(height)) => image.thumbnail_exact(width, height),
            (Some(width), None) => image.thumbnail(width, u32::MAX),
            (None, Some(height)) => image.thumbnail(u32::MAX, height),
            (None, None) => image.resize_exact(
                image.width(),
                (image.height() as f32 * FONT_ASPECT_RATIO) as u32,
                FilterType::Nearest,
            ),
        };

        // Convert to gray-scale
        let image = image.into_luma8();
        let mut string = String::with_capacity((image.width() * image.height()) as usize);
        let coeff = u8::MAX as f32 / (self.characters.len() - 1) as f32;
        for (_, line) in image.enumerate_rows() {
            for (_, _, primitive) in line {
                // TODO: convert
                let luminance = *primitive.channels().first().unwrap();
                let character = self.characters[(luminance as f32 / coeff) as usize];
                string.push(character);
            }
            string.push('\n');
        }

        string
    }
}
