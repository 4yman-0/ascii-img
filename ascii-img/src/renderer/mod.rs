//! Renderer modoule
//! # Example
//! ```
//! use image::DynamicImage;
//! use ascii_img::Renderer;
//! let image = DynamicImage::new_luma8(100, 100);
//! Renderer::default().render(&image);
//! ```

#[cfg(feature = "ansi-renderer")]
mod ansi;

#[cfg(feature = "ansi256-renderer")]
mod ansi256;

#[cfg(feature = "unicode-renderer")]
mod unicode;

mod common;
use alloc::{string::String, vec::Vec};
use image::DynamicImage;

const DEFAULT_CHARS: &[char] = &[' ', '.', '-', ':', '=', '*', '+', '#', '%', '@'];
// TODO: better characters for ANSI and Unicode

/// Abstacts an array of characters, for use in [`Renderer`](struct.Renderer.html)
pub enum RendererCharacters {
    /// The built-in array of characters
    Builtin,
    /// A custom array of characters
    Custom(Vec<char>),
}

impl Default for RendererCharacters {
    fn default() -> Self {
        Self::Builtin
    }
}

#[allow(dead_code)]
impl RendererCharacters {
    /// Return the built-in character array in the form of an empty enum variant
    /// ```
    /// use ascii_img::RendererCharacters;
    /// let renderer_chars = RendererCharacters::builtin();
    /// ```
    pub fn builtin() -> Self {
        Self::Builtin
    }

    /// Returns an instance of `RendererCharacters` from a string-like reference
    /// ```
    /// use ascii_img::RendererCharacters;
    /// let renderer_chars = RendererCharacters::custom_from(" .-#");
    /// ```
    pub fn custom_from<T: AsRef<str>>(string: T) -> Self {
        Self::Custom(string.as_ref().chars().collect())
    }

    /// Returns a `Vec<char>` from contained data.
    /// ```
    /// use ascii_img::RendererCharacters;
    /// let renderer_chars = RendererCharacters::builtin();
    /// println!("{:?}", renderer_chars.get());
    /// ```
    pub fn get(&self) -> Vec<char> {
        match self {
            Self::Builtin => Vec::from(DEFAULT_CHARS),
            Self::Custom(characters) => characters.clone(),
        }
    }
}

#[derive(Clone)]
/// An enum of supported renderer types
pub enum RendererType {
    #[cfg(feature = "ansi-renderer")]
    /// The ANSI renderer
    Ansi,

    #[cfg(feature = "ansi256-renderer")]
    /// The ANSI 256 colors renderer
    Ansi256,

    #[cfg(feature = "unicode-renderer")]
    /// The Unicode renderer
    Unicode,
}

/// Stores generic renderer data
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
            characters: RendererCharacters::builtin(),
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

            #[cfg(feature = "ansi256-renderer")]
            RendererType::Ansi256 => ansi256::render(self, image),

            #[cfg(feature = "unicode-renderer")]
            RendererType::Unicode => unicode::render(self, image),
        }
    }

    pub fn width(&self) -> Option<u32> {
        self.width
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn invert(&self) -> bool {
        self.invert
    }

    pub fn characters(&self) -> &RendererCharacters {
        &self.characters
    }

    pub fn renderer_type(&self) -> &RendererType {
        &self.renderer_type
    }

    pub fn set_width<T: Into<Option<u32>>>(&mut self, width: T) -> &mut Self {
        self.width = width.into();
        self
    }

    pub fn set_height<T: Into<Option<u32>>>(&mut self, height: T) -> &mut Self {
        self.height = height.into();
        self
    }

    pub fn set_invert(&mut self, invert: bool) -> &mut Self {
        self.invert = invert;
        self
    }

    pub fn set_characters(&mut self, characters: RendererCharacters) -> &mut Self {
        self.characters = characters;
        self
    }

    pub fn set_renderer_type(&mut self, renderer_type: RendererType) -> &mut Self {
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
            .set_width(100_u32)
            .set_height(100_u32)
            .set_invert(true)
            .set_characters(RendererCharacters::default())
            .set_renderer_type(RendererType::Unicode);
    }
}
