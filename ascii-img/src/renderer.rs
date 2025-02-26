//! Renderer modoule
//! Defines the `Renderer` enum for listing renderers, the `RendeerTrait` trait for creating renderers and the `RendererOptions` struct for storing geneic renderer data

use image::DynamicImage;

pub trait RendererTrait {
    fn render(self, image: &DynamicImage) -> String;
}

#[derive(Debug, Default)]
pub struct RendererOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
}
