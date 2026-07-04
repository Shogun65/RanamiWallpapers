slint::include_modules!();

mod file_saver;
mod init_file_picker;
mod thumbnail_cache;

use std::{fs, path::Path, process::Command};

use file_saver::file_saver::{read_existing_saved_wallpapers, save_file_2};
use init_file_picker::init::init_a_file_picker;
use shared::log_err::err_log;
use shared::save_wallpaper::SaveWallpaper;
use slint::{ComponentHandle, Image, ModelRc, SharedString, VecModel};
use thumbnail_cache::thumbnail_cache::{
    ffmpeg_command_path, ffprobe_command_path, load_or_create_thumbnail,
};

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    // Populate the library immediately so saved wallpapers appear on launch.
    refresh_wallpaper_library(&ui);

    // Slint callbacks can outlive this stack frame, so keep only a weak UI handle inside them.
    let ui_handle = ui.as_weak();

    ui.on_wallpaper_card_double_clicked(move |wallpaper_path| {
        // Double-clicking a wallpaper card gives you the saved wallpaper path here.
        println!("wallpaper_path: {}", wallpaper_path);
    });

    ui.on_import_wallpaper(move || {
        let ui_handle = ui_handle.clone();

        slint::spawn_local(async move {
            // The file picker is async, so run it on Slint's event loop without freezing the window.
            let file_handle = init_a_file_picker().await;

            if let Some(file_handle) = file_handle {
                if let Err(err) = save_file_2(file_handle) {
                    err_log(&format!("Error on save_file_2: {}", err));
                    return;
                }

                if let Some(ui) = ui_handle.upgrade() {
                    refresh_wallpaper_library(&ui);
                }
            }
        })
        .unwrap();
    });

    return ui.run();
}

fn refresh_wallpaper_library(ui: &AppWindow) {
    // Rebuild the whole card model from the saved JSON file whenever the library changes.
    let saved_wallpapers = match read_existing_saved_wallpapers() {
        Ok(saved_wallpapers) => saved_wallpapers,
        Err(err) => {
            err_log(&format!("Error on read_existing_saved_wallpapers: {}", err));
            ui.set_wallpapers(ModelRc::new(VecModel::from_iter(std::iter::empty::<
                WallpaperCardData,
            >())));
            return;
        }
    };

    ui.set_wallpapers(ModelRc::new(VecModel::from_iter(
        saved_wallpapers.into_iter().map(to_wallpaper_card_data),
    )));
}

fn to_wallpaper_card_data(saved_wallpaper: SaveWallpaper) -> WallpaperCardData {
    // Slint cards use display-friendly strings and an optional cached preview image,
    // so convert the raw saved JSON entry into the UI shape here.
    let wallpaper_path = saved_wallpaper.path.clone();
    let title = wallpaper_title(&saved_wallpaper);
    let format = wallpaper_format(&wallpaper_path);
    // Show user-facing media details on the last line instead of internal cache status text.
    let details = wallpaper_details(&wallpaper_path);
    // Try to attach a cached preview image to the card. If thumbnail generation fails,
    // the UI still works and falls back to a simple placeholder panel.
    let thumbnail = match load_or_create_thumbnail(&wallpaper_path) {
        Ok(thumbnail) => thumbnail,
        Err(err) => {
            err_log(&format!("Error on load_or_create_thumbnail: {}", err));
            Image::default()
        }
    };
    let has_thumbnail = thumbnail.path().is_some();

    WallpaperCardData {
        title: SharedString::from(title),
        path: SharedString::from(wallpaper_path),
        format: SharedString::from(format),
        details: SharedString::from(details),
        thumbnail,
        has_thumbnail,
    }
}

fn wallpaper_title(saved_wallpaper: &SaveWallpaper) -> String {
    // Prefer the saved name, but fall back to the file name when older data is blank or "nan".
    if !saved_wallpaper.name.trim().is_empty() && saved_wallpaper.name.trim() != "nan" {
        return saved_wallpaper.name.clone();
    }

    Path::new(&saved_wallpaper.path)
        .file_stem()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or("Untitled wallpaper")
        .to_string()
}

fn wallpaper_format(path: &str) -> String {
    // The card badge uses the file extension, or a generic label when the path has no extension.
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_uppercase())
        .unwrap_or_else(|| "VIDEO".to_string())
}

fn wallpaper_details(path: &str) -> String {
    let resolution = wallpaper_resolution(path);
    let file_size = wallpaper_file_size(path);

    match (resolution, file_size) {
        (Some(resolution), Some(file_size)) => format!("{resolution} | {file_size}"),
        (Some(resolution), None) => resolution,
        (None, Some(file_size)) => file_size,
        (None, None) => "Saved live wallpaper".to_string(),
    }
}

fn wallpaper_resolution(path: &str) -> Option<String> {
    // ffprobe is made for metadata queries, so use it first and only fall back
    // to parsing ffmpeg output when ffprobe is not available beside the app.
    resolution_from_ffprobe(path).or_else(|| resolution_from_ffmpeg(path))
}

fn resolution_from_ffprobe(path: &str) -> Option<String> {
    let ffprobe_output = Command::new(ffprobe_command_path())
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream=width,height")
        .arg("-of")
        .arg("csv=p=0:s=x")
        .arg(path)
        .output()
        .ok()?;

    if !ffprobe_output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&ffprobe_output.stdout);
    parse_resolution_token(stdout.trim())
}

fn resolution_from_ffmpeg(path: &str) -> Option<String> {
    let ffmpeg_output = Command::new(ffmpeg_command_path())
        .arg("-hide_banner")
        .arg("-i")
        .arg(path)
        .output()
        .ok()?;

    let stderr = String::from_utf8_lossy(&ffmpeg_output.stderr);
    extract_resolution(&stderr)
}

fn extract_resolution(ffmpeg_output: &str) -> Option<String> {
    for token in ffmpeg_output.split(|character: char| {
        character.is_whitespace() || matches!(character, ',' | '[' | ']' | '(' | ')')
    }) {
        if let Some(resolution) = parse_resolution_token(token) {
            return Some(resolution);
        }
    }

    None
}

fn parse_resolution_token(token: &str) -> Option<String> {
    let (width, height) = token.split_once('x')?;

    if !width.chars().all(|character| character.is_ascii_digit())
        || !height.chars().all(|character| character.is_ascii_digit())
    {
        return None;
    }

    let width = width.parse::<u32>().ok()?;
    let height = height.parse::<u32>().ok()?;

    // Ignore tiny values like sample-aspect-ratio hints such as 1x1.
    if width < 100 || height < 100 {
        return None;
    }

    Some(format!("{width}x{height}"))
}

fn wallpaper_file_size(path: &str) -> Option<String> {
    let byte_count = fs::metadata(path).ok()?.len();
    Some(format_file_size(byte_count))
}

fn format_file_size(byte_count: u64) -> String {
    const KIBIBYTE: u64 = 1024;
    const MEBIBYTE: u64 = KIBIBYTE * 1024;
    const GIBIBYTE: u64 = MEBIBYTE * 1024;

    if byte_count >= GIBIBYTE {
        return format!("{:.1} GB", byte_count as f64 / GIBIBYTE as f64);
    }

    if byte_count >= MEBIBYTE {
        return format!("{:.1} MB", byte_count as f64 / MEBIBYTE as f64);
    }

    if byte_count >= KIBIBYTE {
        return format!("{:.1} KB", byte_count as f64 / KIBIBYTE as f64);
    }

    format!("{byte_count} B")
}
