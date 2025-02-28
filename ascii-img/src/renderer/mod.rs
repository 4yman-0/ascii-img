//! Renderer modoule
//! Defines the `Renderer` enum for listing renderers, the `RendeerTrait`
//! trait for creating renderers and the `RendererOptions` struct for storing geneic renderer data

mod ansi;
mod ascii;
mod unicode;
mod common;

use image::DynamicImage;

const DEFAULT_CHAR_VEC: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

pub enum RendererType {
    Ascii,
    Ansi,
    Unicode,
}

#[allow(dead_code)]
impl Renderer {
    pub fn render(&self, image: &DynamicImage) -> String {
        match self.renderer_type {
            RendererType::Ascii => {
                ascii::render(self, &image)
            }
            RendererType::Ansi => {
                ansi::render(self, &image)
            }
            RendererType::Unicode => {
                unicode::render(self, &image)
            }
        }
    }
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
			characters: Vec::from(DEFAULT_CHAR_VEC),
			renderer_type: RendererType::Ascii,
		}
	}
}

#[allow(dead_code)]
impl Renderer {
	pub fn height(mut self, height: Option<u32>) -> Self {
		self.height = height;
		self
	}
		
	pub fn width(mut self, width: Option<u32>) -> Self {
		self.width = width;
		self
	}
	
	pub fn invert(mut self, invert: bool) -> Self {
		self.invert = invert;
		self
	}

	/// Panics if there are too few characters
	pub fn characters(mut self, characters: Vec<char>) -> Self {
		if characters.len() < 2 {
			panic!("Too few characters");
		}
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

    // TODO: tests
}
