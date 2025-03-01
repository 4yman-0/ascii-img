//! Renderer modoule
//! Defines the `Renderer` enum for listing renderers, the `RendeerTrait`
//! trait for creating renderers and the `RendererOptions` struct for storing geneic renderer data

#[cfg(feature = "ansi-renderer")]
mod ansi;

#[cfg(feature = "ascii-renderer")]
mod ascii;

#[cfg(feature = "unicode-renderer")]
mod unicode;

mod common;
use image::DynamicImage;

const ASCII_CHARS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const ANSI_CHARS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const UNICODE_CHARS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn get_characters_for_renderer(renderer_type: &RendererType) -> Vec<char> {
	match renderer_type {
		#[cfg(feature = "ascii-renderer")]
		RendererType::Ascii => {
			Vec::from(ASCII_CHARS)
		},
		#[cfg(feature = "ansi-renderer")]
		RendererType::Ansi => {
			Vec::from(ANSI_CHARS)
		},
		#[cfg(feature = "unicode-renderer")]
		RendererType::Unicode => {
			Vec::from(UNICODE_CHARS)
		},
	}
}

pub enum RendererType {
	#[cfg(feature = "ascii-renderer")]
    Ascii,

    #[cfg(feature = "ansi-renderer")]
    Ansi,

    #[cfg(feature = "unicode-renderer")]
    Unicode,
}

#[allow(dead_code)]
impl Renderer {
    pub fn render(&self, image: &DynamicImage) -> String {
        match self.renderer_type {
        	#[cfg(feature = "ascii-renderer")]
            RendererType::Ascii => {
                ascii::render(self, image)
            },
            
            #[cfg(feature = "ansi-renderer")]
            RendererType::Ansi => {
                ansi::render(self, image)
            },
            
            #[cfg(feature = "unicode-renderer")]
            RendererType::Unicode => {
                unicode::render(self, image)
            },
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
			characters: get_characters_for_renderer(&RendererType::Ascii),
			renderer_type: RendererType::Ascii,
		}
	}
}

#[allow(dead_code)]
impl Renderer {
	pub fn new(
	    width: Option<u32>,
	    height: Option<u32>,
	    invert: bool,
	    characters: Option<Vec<char>>,
	    renderer_type: RendererType
	) -> Self {
		if let Some(chars) = &characters {
			if chars.is_empty() {
				panic!("No characters to render with: Please use `None`");
			}
		}
		Self {
			width,
			height,
			invert,
			characters: characters
				.unwrap_or(
					get_characters_for_renderer(&renderer_type)
				),
			renderer_type,
		}
	}
}
