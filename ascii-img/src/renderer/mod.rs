//! Renderer modoule
//! Defines the `RendererType` enum for listing renderers and the `Renderer` struct for storing generic renderer data
//! # Example
//! ```
//! use image::DynamicImage;
//! use ascii_img::RendererConfig;
//! let image = DynamicImage::new_luma8(100, 100);
//! RendererConfig::default().render(&image);
//! ```

#[cfg(feature = "ansi-renderer")]
mod ansi;

#[cfg(feature = "ansi256-renderer")]
mod ansi256;

#[cfg(feature = "unicode-renderer")]
mod unicode;

mod common;
use alloc::{string::String, vec::Vec};
use image::{DynamicImage, Rgb};

const DEFAULT_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];
// TODO: better characters for ANSI and Unicode

pub trait Renderer {
    /// Render a single pixel as a colored character
    fn render_pixel(&self, pixel: &Rgb<u8>, characters: &[char], coeff: f32) -> (String, char);
    
    /// Process the image before rendering
    fn preprocess_image(&self, image: &DynamicImage, config: &RendererConfig) -> DynamicImage {
    	let mut image = common::resize(image, config);
    	if config.invert {
    	    image.invert();
    	};

    	image
    }

    /// Render the image to ASCII art
    fn render(&self, image: &DynamicImage, config: &RendererConfig) -> String {
        let image = self.preprocess_image(image, config).to_rgb8();
        
        let mut string = self.create_output_buffer(image.width(), image.height());
        let char_array = config.characters.get();
        let coeff = u8::MAX as f32 / (char_array.len() - 1) as f32;
        let mut last_pixel: Option<Rgb<u8>> = None;

        for line in image.rows() {
            for pixel in line {
                if last_pixel != Some(*pixel) {
                    let (color_code, character) = self.render_pixel(pixel, &char_array, coeff);
                    string.push_str(&color_code);
                    string.push(character);
                    last_pixel = Some(*pixel);
                } else {
                    let (_, character) = self.render_pixel(pixel, &char_array, coeff);
                    string.push(character);
                }
            }
            string.push('\n');
        }
        
        string
    }

    /// Create an empty string buffer with the appropriate capacity
    fn create_output_buffer(&self, width: u32, height: u32) -> String {
        String::with_capacity((width * height * 2) as usize)
    }
}

pub enum RendererCharacters {
    Builtin,
    String(Vec<char>),
}

impl Default for RendererCharacters {
    fn default() -> Self {
        Self::Builtin
    }
}

#[allow(dead_code)]
impl RendererCharacters {
    /// Return the built-in characters in the form of a zero-sized enum variant
    /// ```
    /// use ascii_img::RendererCharacters;
    /// let renderer_chars = RendererCharacters::builtin();
    /// ```
    pub fn builtin() -> Self {
        Self::Builtin
    }

    /// Returns a new `Vec<char>` from the string
    /// ```
    /// use ascii_img::RendererCharacters;
    /// let renderer_chars = RendererCharacters::from_string(" .-#");
    /// ```
    pub fn from_string(string: &str) -> Self {
        Self::String(string.chars().collect())
    }

    /// Creates a new `Vec<char>` from contained data.
    /// ```
    /// use ascii_img::RendererCharacters;
    /// let renderer_chars = RendererCharacters::builtin();
    /// println!("{:?}", renderer_chars.get());
    /// ```
    pub fn get(&self) -> Vec<char> {
        match self {
            Self::Builtin => Vec::from(DEFAULT_CHARS),
            Self::String(characters) => characters.clone(),
        }
    }
}

#[derive(Clone)]
pub enum RendererType {
    #[cfg(feature = "ansi-renderer")]
    Ansi,

    #[cfg(feature = "ansi256-renderer")]
    Ansi256,

    #[cfg(feature = "unicode-renderer")]
    Unicode,
}

pub struct RendererConfig {
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    characters: RendererCharacters,
    renderer_type: RendererType,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            invert: false,
            characters: RendererCharacters::builtin(),
            renderer_type: RendererType::Unicode,
        }
    }
}

#[allow(dead_code)]
impl RendererConfig {
    /// Renders an image into a string
    pub fn render(&self, image: &DynamicImage) -> String {
        match self.renderer_type {
            #[cfg(feature = "ansi-renderer")]
            RendererType::Ansi => ansi::AnsiRenderer.render(image, self),

            #[cfg(feature = "ansi256-renderer")]
            RendererType::Ansi256 => ansi256::Ansi256Renderer.render(image, self),

            #[cfg(feature = "unicode-renderer")]
            RendererType::Unicode => unicode::UnicodeRenderer.render(image, self),
        }
    }

    pub fn width(mut self, width: Option<u32>) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Option<u32>) -> Self {
        self.height = height;
        self
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    pub fn characters(mut self, characters: RendererCharacters) -> Self {
        self.characters = characters;
        self
    }

    pub fn renderer_type(mut self, renderer_type: RendererType) -> Self {
        self.renderer_type = renderer_type;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn renderer_config_test() {
        let _renderer = RendererConfig::default()
            .width(Some(100))
            .height(Some(100))
            .invert(true)
            .characters(RendererCharacters::default())
            .renderer_type(RendererType::Unicode);
    }
}
