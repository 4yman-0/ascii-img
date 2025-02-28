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
    let renderer = Renderer::default()
    	.width(cli.width)
    	.height(cli.height)
    	.invert(cli.invert.unwrap_or(false));
    //if let Some(characters) = cli.characters {
    //	renderer.characters(characters.chars().collect())
    //}
    //if let Some(renderer_type) = cli.renderer_type {
     //	renderer.characters(RendererType::from(renderer_type))
    //}

    Ok(renderer.render(&image))
}
