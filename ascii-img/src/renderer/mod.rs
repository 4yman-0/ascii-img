//! Renderer modoule
//! Defines the `Renderer` enum for listing renderers and the `Renderer` struct for storing geneic renderer data
//! # Example
//! ```
//! use image::DynamicImage;
//! use ascii_img::Renderer;
//! let image = DynamicImage::new_luma8(100, 100);
//! Renderer::default().render(&image);
//! ```

#[cfg(feature = "ansi-renderer")]
mod ansi;

#[cfg(feature = "unicode-renderer")]
mod unicode;

mod common;
use alloc::{string::String, vec::Vec};
use image::DynamicImage;

const ASCII_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];
const ANSI_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];
const UNICODE_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];

#[derive(Default, Clone)]
pub enum RendererCharactersType {
    #[default]
    Ascii,
    Ansi,
    Unicode,
}

pub enum RendererCharacters {
    Builtin(RendererCharactersType),
    Custom(Vec<char>),
}

impl Default for RendererCharacters {
    fn default() -> Self {
        Self::Builtin(RendererCharactersType::default())
    }
}

#[allow(dead_code)]
impl RendererCharacters {
    pub fn from_type(chars_type: RendererCharactersType) -> Self {
        Self::Builtin(chars_type)
    }

    pub fn from_string(string: &str) -> Self {
        Self::Custom(string.chars().collect())
    }

    /// Creates a new `Vec<char>` from contained data.
    pub fn get(&self) -> Vec<char> {
        match self {
            Self::Builtin(chars_type) => Vec::from(match chars_type {
                RendererCharactersType::Ascii => ASCII_CHARS,
                RendererCharactersType::Ansi => ANSI_CHARS,
                RendererCharactersType::Unicode => UNICODE_CHARS,
            }),
            Self::Custom(characters) => characters.clone(),
        }
    }
}

#[derive(Clone)]
pub enum RendererType {
    #[cfg(feature = "ansi-renderer")]
    Ansi,

    #[cfg(feature = "unicode-renderer")]
    Unicode,
}

pub struct Renderer {
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    characters: RendererCharacters,
    renderer_type: RendererType,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            invert: false,
            characters: RendererCharacters::from_type(RendererCharactersType::Ascii),
            renderer_type: RendererType::Unicode,
        }
    }
}

#[allow(dead_code)]
impl Renderer {
    /// Renders an image into a string
    pub fn render(&self, image: &DynamicImage) -> String {
        match self.renderer_type {
            #[cfg(feature = "ansi-renderer")]
            RendererType::Ansi => ansi::render(self, image),

            #[cfg(feature = "unicode-renderer")]
            RendererType::Unicode => unicode::render(self, image),
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
    fn renderer_build_test() {
        let _renderer = Renderer::default()
            .width(Some(100))
            .height(Some(100))
            .invert(true)
            .characters(RendererCharacters::default())
            .renderer_type(RendererType::Unicode);
    }
}
