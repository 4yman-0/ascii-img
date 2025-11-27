mod cli;

use ascii_img::{RendererConfig, RendererCharacters};
use clap::Parser;
use cli::Cli;
use image::ImageError;

fn main() {
    let cli = Cli::parse();
    let art = render(cli).expect("Error while rendering");
    print!("{art}");
}

fn render(cli: Cli) -> Result<String, ImageError> {
    let image = image::open(cli.image_path)?;
    let characters = {
        if let Some(characters) = cli.characters {
            if characters == "builtin" {
                RendererCharacters::builtin()
            } else {
                RendererCharacters::custom_from(characters.as_str())
            }
        } else {
            RendererCharacters::default()
        }
    };
    let result = RendererConfig::default()
        .set_width(cli.width)
        .set_height(cli.height)
        .set_invert(cli.invert.unwrap_or(false))
        .set_characters(characters)
        .set_renderer_type(cli.renderer_type.unwrap_or_default().into())
        .render(&image);

    Ok(result)
}
