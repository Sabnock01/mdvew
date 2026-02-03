mod display;
mod error;
mod render;

use clap::{Parser, ValueEnum};
use std::{env, fs, path::PathBuf};

use error::{MdvewError, Result};

const HTML_PATH: &str = "/tmp/mdvew.html";

fn home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        env::var("HOME").ok().map(PathBuf::from)
    }
    #[cfg(windows)]
    {
        env::var("USERPROFILE").ok().map(PathBuf::from)
    }
}

fn browser_html_path() -> PathBuf {
    home_dir()
        .map(|home| home.join("mdvew").join("preview.html"))
        .unwrap_or_else(|| PathBuf::from("/tmp/mdvew-preview.html"))
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

#[derive(Parser)]
#[command(name = "mdvew")]
#[command(about = "Render markdown files as images in the terminal")]
struct Cli {
    /// Path to the markdown file
    file_path: PathBuf,

    /// Save PNG to file instead of displaying
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Viewport width in pixels (default: auto-scale to terminal)
    #[arg(short = 'w', long)]
    viewport_width: Option<u32>,

    /// Open in system browser
    #[arg(short, long)]
    browser: bool,

    /// Color theme
    #[arg(short, long, value_enum, default_value_t = Theme::Light)]
    theme: Theme,
}

const PIXELS_PER_COLUMN: u32 = 8;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let markdown_text = fs::read_to_string(&cli.file_path).map_err(|source| {
        MdvewError::ReadFile {
            path: cli.file_path.clone(),
            source,
        }
    })?;

    let full_html = render::build_full_html(&markdown_text, &cli.file_path);
    fs::write(HTML_PATH, &full_html)?;

    if cli.browser {
        let browser_path = browser_html_path();
        if let Some(parent_dir) = browser_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        fs::write(&browser_path, &full_html)?;
        display::open_in_browser(&browser_path);
        return Ok(());
    }

    let viewport_width = cli.viewport_width.unwrap_or_else(|| {
        let (term_cols, _) = viuer::terminal_size();
        (term_cols as u32 * PIXELS_PER_COLUMN).min(1200)
    });

    let screenshot_bytes =
        render::capture_screenshot(HTML_PATH.as_ref(), viewport_width, cli.theme)?;

    if let Some(ref output_path) = cli.output {
        return display::save_to_file(&screenshot_bytes, output_path);
    }

    display::show_in_terminal(&screenshot_bytes)
}
