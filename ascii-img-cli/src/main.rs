mod cli;

use ascii_img::{Renderer, RendererCharacters, RendererCharactersType};
use clap::Parser;
use cli::Cli;
use image::ImageError;

fn main() {
    let cli = Cli::parse();
    let art = render(cli).expect("Error while rendering");
    print!("{}", art);
}

fn render(cli: Cli) -> Result<String, ImageError> {
    let image = image::open(cli.image_path)?;
    let characters = {
        if let Some(characters) = cli.characters {
            if characters == "ascii" {
                RendererCharacters::from_type(RendererCharactersType::Ascii)
            } else if characters == "ansi" {
                RendererCharacters::from_type(RendererCharactersType::Ansi)
            } else if characters == "unicode" {
                RendererCharacters::from_type(RendererCharactersType::Unicode)
            } else {
                RendererCharacters::from_string(characters.as_str())
            }
        } else {
            RendererCharacters::default()
        }
    };
    let renderer = Renderer::default()
        .width(cli.width)
        .height(cli.height)
        .invert(cli.invert.unwrap_or(false))
        .characters(characters)
        .renderer_type(cli.renderer_type.unwrap_or_default().into());

    Ok(renderer.render(&image))
}
