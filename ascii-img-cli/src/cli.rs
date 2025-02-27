use clap::Parser;

/// Command-line tool for using `ascii-img`
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub image_path: String,

    #[arg(long)]
    pub width: Option<u32>,

    #[arg(long)]
    pub height: Option<u32>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cli() {
        let cli = Cli::parse_from(vec!["path/to/bin", "path/to/image"]);
        assert_eq!(cli.width, None);
        assert_eq!(cli.height, None);
    }
}
