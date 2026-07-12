#![windows_subsystem = "windows"]
slint::include_modules!();

mod file_saver;
mod init_file_picker;
mod thumbnail_cache;
mod namepipe;

use std::{
    fs,
    path::Path,
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

use file_saver::file_saver::{read_existing_saved_wallpapers, save_file_2};
use init_file_picker::init::init_a_file_picker;
use shared::log_err::err_log;
use shared::save_wallpaper::SaveWallpaper;
use namepipe::namepipe::{get_runtime, sent_struct_of_data_to_client};

use slint::{
    ComponentHandle, Image, Model, ModelRc, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel,
};
use thumbnail_cache::thumbnail_cache::{
    cleanup_stale_thumbnails, ffmpeg_command_path, ffprobe_command_path,
    load_or_create_thumbnail_path,
};

fn main() -> Result<(), slint::PlatformError> {
    
    let ui = AppWindow::new()?;

    /* 
        lets get the runtime here and his handle
    */

    let runtime = get_runtime().unwrap();// get panic for now maybe for forever who knows
    let handle = runtime.handle().clone();

    let refresh_generation = Arc::new(AtomicU64::new(0));
    // Populate the library immediately so saved wallpapers appear on launch.
    refresh_wallpaper_library(&ui, &refresh_generation);

    // Slint callbacks can outlive this stack frame, so keep only a weak UI handle inside them.
    let ui_handle = ui.as_weak();
    let refresh_generation_for_import = Arc::clone(&refresh_generation);

    ui.on_wallpaper_card_double_clicked(move |wallpaper_path| {
        // Double-clicking a wallpaper card gives you the saved wallpaper path here
        println!("[DEBUG] wallpaper_path: {}", wallpaper_path);
        sent_struct_of_data_to_client(wallpaper_path.to_string(), &handle);
        println!("[DEBUG] sent the data!");
    });

    ui.on_import_wallpaper(move || {
        let ui_handle = ui_handle.clone();
        let refresh_generation_for_import = Arc::clone(&refresh_generation_for_import);

        slint::spawn_local(async move {
            // The file picker is async, so run it on Slint's event loop without freezing the window.
            let file_handle = init_a_file_picker().await;

            if let Some(file_handle) = file_handle {
                if let Err(err) = save_file_2(file_handle) {
                    err_log(&format!("Error on save_file_2: {}", err));
                    return;
                }

                if let Some(ui) = ui_handle.upgrade() {
                    refresh_wallpaper_library(&ui, &refresh_generation_for_import);
                }
            }
        })
        .unwrap();
    });

    return ui.run();
}

fn refresh_wallpaper_library(ui: &AppWindow, refresh_generation: &Arc<AtomicU64>) {
    // Each refresh gets its own generation number so older worker threads stop updating
    // the UI after a newer import or reload replaces the model.
    let generation = refresh_generation.fetch_add(1, Ordering::SeqCst) + 1;

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

    let wallpaper_paths = saved_wallpapers
        .iter()
        .map(|wallpaper| wallpaper.path.clone())
        .collect::<Vec<_>>();

    // Show lightweight placeholder cards first, then fill in thumbnails/details from a worker thread.
    let placeholder_cards = saved_wallpapers
        .iter()
        .map(placeholder_wallpaper_card_data)
        .collect::<Vec<_>>();
    ui.set_wallpapers(ModelRc::new(VecModel::from(placeholder_cards)));

    let ui_handle = ui.as_weak();
    let refresh_generation = Arc::clone(refresh_generation);

    thread::spawn(move || {
        if let Err(err) = cleanup_stale_thumbnails(&wallpaper_paths) {
            err_log(&format!("Error on cleanup_stale_thumbnails: {}", err));
        }

        for (index, saved_wallpaper) in saved_wallpapers.into_iter().enumerate() {
            if refresh_generation.load(Ordering::SeqCst) != generation {
                return;
            }

            let resolved_card = resolve_wallpaper_card(saved_wallpaper);

            if refresh_generation.load(Ordering::SeqCst) != generation {
                return;
            }

            let refresh_generation_for_ui = Arc::clone(&refresh_generation);

            if ui_handle
                .upgrade_in_event_loop(move |ui| {
                    if refresh_generation_for_ui.load(Ordering::SeqCst) != generation {
                        return;
                    }

                    let wallpaper_model = ui.get_wallpapers();
                    let Some(wallpaper_model) = wallpaper_model
                        .as_any()
                        .downcast_ref::<VecModel<WallpaperCardData>>()
                    else {
                        return;
                    };

                    wallpaper_model.set_row_data(index, resolved_card.into_wallpaper_card_data());
                })
                .is_err()
            {
                return;
            }
        }

        if refresh_generation.load(Ordering::SeqCst) != generation {
            return;
        }

        if let Err(err) = cleanup_stale_thumbnails(&wallpaper_paths) {
            err_log(&format!("Error on cleanup_stale_thumbnails: {}", err));
        }
    });
}

fn placeholder_wallpaper_card_data(saved_wallpaper: &SaveWallpaper) -> WallpaperCardData {
    let wallpaper_path = saved_wallpaper.path.clone();
    let title = wallpaper_title(&saved_wallpaper);
    let format = wallpaper_format(&wallpaper_path);

    WallpaperCardData {
        title: SharedString::from(title),
        path: SharedString::from(wallpaper_path),
        format: SharedString::from(format),
        details: SharedString::from("Loading wallpaper info..."),
        thumbnail: Image::default(),
        has_thumbnail: false,
        is_loading: true,
    }
}

fn resolve_wallpaper_card(saved_wallpaper: SaveWallpaper) -> ResolvedWallpaperCardData {
    // The expensive work (ffprobe + ffmpeg + filesystem metadata) happens on a worker thread,
    // and the UI thread only receives the finished row data afterward.
    let wallpaper_path = saved_wallpaper.path.clone();
    let title = wallpaper_title(&saved_wallpaper);
    let format = wallpaper_format(&wallpaper_path);
    let details = wallpaper_details(&wallpaper_path);
    let thumbnail_buffer = match load_or_create_thumbnail_path(&wallpaper_path) {
        Ok(thumbnail_path) => decode_thumbnail_buffer_from_path(&thumbnail_path),
        Err(err) => {
            err_log(&format!("Error on load_or_create_thumbnail_path: {}", err));
            None
        }
    };

    ResolvedWallpaperCardData {
        title,
        path: wallpaper_path,
        format,
        details,
        thumbnail_buffer,
    }
}

fn decode_thumbnail_buffer_from_path(path: &Path) -> Option<SharedPixelBuffer<Rgba8Pixel>> {
    let rgba_image = image::open(path)
        .map_err(|err| format!("Could not decode thumbnail image {}: {err}", path.display()))
        .map_or_else(
            |err| {
                err_log(&err);
                None
            },
            Some,
        )?
        .into_rgba8();

    let (width, height) = rgba_image.dimensions();
    let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    pixel_buffer
        .make_mut_bytes()
        .copy_from_slice(rgba_image.as_raw());

    Some(pixel_buffer)
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

struct ResolvedWallpaperCardData {
    title: String,
    path: String,
    format: String,
    details: String,
    thumbnail_buffer: Option<SharedPixelBuffer<Rgba8Pixel>>,
}

impl ResolvedWallpaperCardData {
    fn into_wallpaper_card_data(self) -> WallpaperCardData {
        // Slint::Image is not Send, so the worker thread forwards only plain data and a
        // decoded pixel buffer. The final Image object is created back on the UI thread here.
        let thumbnail = self.thumbnail_buffer.map(Image::from_rgba8);
        let has_thumbnail = thumbnail.is_some();

        WallpaperCardData {
            title: SharedString::from(self.title),
            path: SharedString::from(self.path),
            format: SharedString::from(self.format),
            details: SharedString::from(self.details),
            thumbnail: thumbnail.unwrap_or_default(),
            has_thumbnail,
            is_loading: false,
        }
    }
}
