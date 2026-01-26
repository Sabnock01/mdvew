use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MdvewError {
    #[error("Failed to read file {path}: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write file {path}: {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(
        "Failed to launch browser: {0}\n\
         Ensure Chrome/Chromium is installed.\n\
         Debian/Ubuntu: sudo apt install chromium-browser\n\
         Arch: sudo pacman -S chromium\n\
         macOS: brew install --cask chromium"
    )]
    BrowserLaunch(String),

    #[error("Browser error: {0}")]
    Browser(String),

    #[error("Failed to decode screenshot: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Failed to decode image: {0}")]
    ImageDecode(#[from] image::ImageError),

    #[error("Failed to display image: {0}")]
    Display(#[from] viuer::ViuError),
}

pub type Result<T> = std::result::Result<T, MdvewError>;
