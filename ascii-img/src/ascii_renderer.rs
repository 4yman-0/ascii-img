//! ASCII renderer module
#[allow(dead_code)]
use crate::renderer::{RendererOptions, RendererTrait};
use image::DynamicImage;

/// ASCII renderer
/// Renders using ASCII characters only
#[derive(Debug, Default)]
pub struct AsciiRenderer {
    options: RendererOptions,
}

#[allow(dead_code)]
impl AsciiRenderer {
    /// Creates a new `AsciiRenderer`
    fn new() -> Self {
        Self::default()
    }

    /// Sets the  width
    fn width(mut self, width: Option<u32>) -> Self {
        self.options.width = width;
        self
    }

    /// Sets the height
    fn height(mut self, height: Option<u32>) -> Self {
        self.options.height = height;
        self
    }
}

impl RendererTrait for AsciiRenderer {
    fn render(self, _image: &DynamicImage) -> String {
        unimplemented!("Implement ASCII Renderer");
    }
}
