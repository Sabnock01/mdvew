use std::path::Path;
use std::process::Command;

use crate::error::{MdvewError, Result};

pub fn save_to_file(screenshot_bytes: &[u8], output_path: &Path) -> Result<()> {
    std::fs::write(output_path, screenshot_bytes).map_err(|source| MdvewError::WriteFile {
        path: output_path.to_path_buf(),
        source,
    })?;
    println!("Saved to {}", output_path.display());
    Ok(())
}

pub fn open_in_browser(html_path: &Path) {
    let path_str = html_path.display().to_string();

    #[cfg(target_os = "linux")]
    let _ = Command::new("xdg-open").arg(&path_str).spawn();

    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(&path_str).spawn();

    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd")
        .args(["/C", "start", &path_str])
        .spawn();
}

const MAX_DISPLAY_COLS: u16 = 150;

pub fn show_in_terminal(screenshot_bytes: &[u8]) -> Result<()> {
    let img = image::load_from_memory(screenshot_bytes)?;

    let (term_width, _) = viuer::terminal_size();
    let display_width = term_width.min(MAX_DISPLAY_COLS);

    let config = viuer::Config {
        width: Some(display_width as u32),
        absolute_offset: false,
        ..Default::default()
    };

    viuer::print(&img, &config)?;
    Ok(())
}
