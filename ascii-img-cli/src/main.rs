mod cli;

use ascii_img::{ascii_renderer::AsciiRenderer, renderer::RendererTrait};
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let image = match image::open(cli.image_path) {
        Ok(image) => image,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };
    let renderer = AsciiRenderer::new().width(cli.width).height(cli.height);
    let art = renderer.render(&image);
    print!("{}", art);
    Ok(())
}
