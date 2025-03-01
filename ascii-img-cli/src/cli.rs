use clap::Parser;
use ascii_img::renderer::RendererType;

#[derive(clap::ValueEnum, Clone, Default)]
pub enum RendererTypeArg {
	/// ASCII renderer
	#[default]
	Ascii,
	/// ANSI renderer
	Ansi,
	/// Unicode renderer
	Unicode,
}

impl From<RendererTypeArg> for RendererType {
	fn from(renderer_type: RendererTypeArg) -> Self {
		match renderer_type {
			RendererTypeArg::Ascii => Self::Ascii,
			RendererTypeArg::Ansi => Self::Ansi,
			RendererTypeArg::Unicode => Self::Unicode,
		}
	}
}

/// Command-line tool for using `ascii-img`
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub image_path: String,

    #[arg(long)]
    pub width: Option<u32>,

    #[arg(long)]
    pub height: Option<u32>,

    #[arg(short, long)]
    pub invert: Option<bool>,

    #[arg(short, long)]
    pub characters: Option<String>,

    #[arg(short, long, value_enum)]
    pub renderer_type: Option<RendererTypeArg>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cli() {
        let cli = Cli::parse_from(vec!["path/to/bin", "path/to/image"]);
        assert_eq!(cli.image_path, "path/to/image".to_string());
        assert!(!cli.width.is_some());
		assert!(!cli.height.is_some());
        assert!(!cli.invert.is_some());
        assert!(!cli.characters.is_some());
        assert!(!cli.renderer_type.is_some());
    }
}
