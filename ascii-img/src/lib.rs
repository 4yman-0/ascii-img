//! Convert images to ASCII
#![no_std]
extern crate alloc;

pub mod renderer;
pub use renderer::{RendererConfig, Renderer, RendererCharacters, RendererType};
