use base64::{engine::general_purpose, Engine as _};
use headless_chrome::{
    protocol::cdp::{Emulation, Page},
    Browser, LaunchOptions,
};
use pulldown_cmark::{html, Options, Parser as MdParser};
use std::path::{Path, PathBuf};

use crate::error::{MdvewError, Result};
use crate::Theme;

const GITHUB_CSS: &str = include_str!("github-markdown.css");

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    {{BASE_TAG}}
    <style>
        {{CSS}}
        body {
            box-sizing: border-box;
            min-width: 200px;
            max-width: 980px;
            margin: 0 auto;
            padding: 45px;
        }
    </style>
</head>
<body class="markdown-body">
    <article>
        {{CONTENT}}
    </article>
</body>
</html>"#;

pub fn markdown_to_html(markdown_text: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES;

    let parser = MdParser::new_ext(markdown_text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn build_full_html(markdown_text: &str, file_path: &PathBuf) -> String {
    let html_content = markdown_to_html(markdown_text);
    let base_tag = build_base_tag(file_path);

    HTML_TEMPLATE
        .replace("{{BASE_TAG}}", &base_tag)
        .replace("{{CSS}}", GITHUB_CSS)
        .replace("{{CONTENT}}", &html_content)
}

fn build_base_tag(file_path: &PathBuf) -> String {
    file_path
        .canonicalize()
        .ok()
        .and_then(|canonical_path| {
            canonical_path
                .parent()
                .map(|parent_dir| format!(r#"<base href="file://{}/">"#, parent_dir.display()))
        })
        .unwrap_or_default()
}

pub fn capture_screenshot(html_path: &Path, viewport_width: u32, theme: Theme) -> Result<Vec<u8>> {
    let launch_options = LaunchOptions {
        headless: true,
        window_size: Some((viewport_width, 800)),
        args: vec![std::ffi::OsStr::new("--force-color-profile=srgb")],
        ..Default::default()
    };

    let browser = Browser::new(launch_options)
        .map_err(|err| MdvewError::BrowserLaunch(err.to_string()))?;

    let tab = browser
        .new_tab()
        .map_err(|err| MdvewError::Browser(err.to_string()))?;

    let _ = tab.call_method(Emulation::SetEmulatedMedia {
        media: Some("screen".into()),
        features: Some(vec![Emulation::MediaFeature {
            name: "prefers-color-scheme".into(),
            value: theme.as_str().into(),
        }]),
    });

    let file_url = format!("file://{}", html_path.display());
    tab.navigate_to(&file_url)
        .map_err(|err| MdvewError::Browser(err.to_string()))?;

    tab.wait_for_element("body")
        .map_err(|err| MdvewError::Browser(err.to_string()))?;

    let (content_width, content_height) = get_content_dimensions(&tab)?;

    let clip = Page::Viewport {
        x: 0.0,
        y: 0.0,
        width: content_width,
        height: content_height,
        scale: 1.0,
    };

    let screenshot_response = tab
        .call_method(Page::CaptureScreenshot {
            format: Some(Page::CaptureScreenshotFormatOption::Png),
            quality: None,
            clip: Some(clip),
            from_surface: Some(true),
            capture_beyond_viewport: Some(true),
            optimize_for_speed: None,
        })
        .map_err(|err| MdvewError::Browser(err.to_string()))?;

    let screenshot_bytes = general_purpose::STANDARD.decode(screenshot_response.data)?;
    Ok(screenshot_bytes)
}

fn get_content_dimensions(tab: &headless_chrome::Tab) -> Result<(f64, f64)> {
    let js_code = "JSON.stringify([document.body.scrollWidth, document.body.scrollHeight])";

    let eval_result = tab
        .evaluate(js_code, false)
        .map_err(|err| MdvewError::Browser(err.to_string()))?;

    let dimensions_str = eval_result
        .value
        .as_ref()
        .and_then(|val| val.as_str())
        .unwrap_or("[800, 600]");

    Ok(parse_dimensions(dimensions_str))
}

fn parse_dimensions(json_str: &str) -> (f64, f64) {
    let trimmed = json_str
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    let mut parts = trimmed.split(',');

    let width = parts
        .next()
        .and_then(|s| s.trim().parse::<f64>().ok())
        .unwrap_or(800.0);

    let height = parts
        .next()
        .and_then(|s| s.trim().parse::<f64>().ok())
        .unwrap_or(600.0);

    (width, height)
}
