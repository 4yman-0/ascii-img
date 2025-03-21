use std::path::PathBuf;

use ascii_img::renderer::RendererType;
use clap::Parser;

#[derive(clap::ValueEnum, Clone, Default)]
pub enum RendererTypeArg {
    /// ANSI renderer
    Ansi,
    /// Unicode renderer
    #[default]
    Unicode,
}

impl From<RendererTypeArg> for RendererType {
    fn from(renderer_type: RendererTypeArg) -> Self {
        match renderer_type {
            RendererTypeArg::Ansi => Self::Ansi,
            RendererTypeArg::Unicode => Self::Unicode,
        }
    }
}

/// Command-line tool for using `ascii-img`
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub image_path: PathBuf,

    #[arg(long)]
    pub width: Option<u32>,

    #[arg(long)]
    pub height: Option<u32>,

    #[arg(short, long)]
    pub invert: Option<bool>,

    #[arg(short, long)]
    pub characters: Option<String>,

    #[arg(short, long = "renderer", value_enum)]
    pub renderer_type: Option<RendererTypeArg>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cli_test() {
        let cli = Cli::parse_from(vec!["path/to/bin", "path/to/image"]);
        assert_eq!(cli.image_path, PathBuf::from("path/to/image"));
        assert!(!cli.width.is_some());
        assert!(!cli.height.is_some());
        assert!(!cli.invert.is_some());
        assert!(!cli.characters.is_some());
        assert!(!cli.renderer_type.is_some());
    }
}
