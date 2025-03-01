mod cli;

use ascii_img::Renderer;
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
    let characters = cli.characters.map(|chars | chars.chars().collect());
    let renderer = Renderer::new(
    	cli.width,
    	cli.height,
    	cli.invert.unwrap_or(false),
    	characters,
    	cli.renderer_type.unwrap_or_default().into()
    );

    Ok(renderer.render(&image))
}
