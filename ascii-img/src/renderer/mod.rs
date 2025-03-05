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

#[cfg(feature = "ascii-renderer")]
mod ascii;

#[cfg(feature = "unicode-renderer")]
mod unicode;

mod common;
use image::DynamicImage;

const ASCII_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];
const ANSI_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];
const UNICODE_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];

fn get_characters_for_renderer(renderer_type: &RendererType) -> Vec<char> {
    Vec::from(match renderer_type {
        #[cfg(feature = "ascii-renderer")]
        RendererType::Ascii => ASCII_CHARS,
        #[cfg(feature = "ansi-renderer")]
        RendererType::Ansi => ANSI_CHARS,
        #[cfg(feature = "unicode-renderer")]
        RendererType::Unicode => UNICODE_CHARS,
    })
}

pub enum RendererType {
    #[cfg(feature = "ascii-renderer")]
    Ascii,

    #[cfg(feature = "ansi-renderer")]
    Ansi,

    #[cfg(feature = "unicode-renderer")]
    Unicode,
}

pub struct Renderer {
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    pub characters: Vec<char>,
    renderer_type: RendererType,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            invert: false,
            characters: get_characters_for_renderer(&RendererType::Ascii),
            renderer_type: RendererType::Ascii,
        }
    }
}

#[allow(dead_code)]
impl Renderer {
    /// Renders an image into a string
    pub fn render(&self, image: &DynamicImage) -> String {
        match self.renderer_type {
            #[cfg(feature = "ascii-renderer")]
            RendererType::Ascii => ascii::render(self, image),

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

    pub fn characters(mut self, characters: Option<Vec<char>>) -> Self {
        self.characters = characters.unwrap_or(get_characters_for_renderer(&self.renderer_type));
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
            .characters(Some(vec!['#', 'w', ' ']))
            .renderer_type(RendererType::Ascii);
    }
}
