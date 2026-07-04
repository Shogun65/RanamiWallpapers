slint::include_modules!();

mod file_saver;
mod init_file_picker;
mod thumbnail_cache;

use std::path::Path;

use file_saver::file_saver::{read_saved_wallpapers, save_file_2};
use init_file_picker::init::init_a_file_picker;
use shared::log_err::err_log;
use shared::save_wallpaper::SaveWallpaper;
use slint::{ComponentHandle, Image, ModelRc, SharedString, VecModel};
use thumbnail_cache::thumbnail_cache::load_or_create_thumbnail;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    refresh_wallpaper_library(&ui);

    let ui_handle = ui.as_weak();

    ui.on_wallpaper_card_double_clicked(|wallpaper_path| {
        // Double-clicking a wallpaper card gives you the saved wallpaper path here.
        // Use `wallpaper_path` however you want for the low-level wallpaper logic.
        let a = wallpaper_path;
        println!("wallpaper_path: {}", a);
    });

    ui.on_import_wallpaper(move || {
        let ui_handle = ui_handle.clone();

        slint::spawn_local(async move {
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
    let saved_wallpapers = match read_saved_wallpapers() {
        Ok(saved_wallpapers) => saved_wallpapers,
        Err(err) => {
            err_log(&format!("Error on read_saved_wallpapers: {}", err));
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
    let title = wallpaper_title(&saved_wallpaper);
    let format = wallpaper_format(&saved_wallpaper.path);
    // Try to attach a cached preview image to the card. If thumbnail generation fails,
    // the UI still works and falls back to a simple placeholder panel.
    let thumbnail = match load_or_create_thumbnail(&saved_wallpaper.path) {
        Ok(thumbnail) => thumbnail,
        Err(err) => {
            err_log(&format!("Error on load_or_create_thumbnail: {}", err));
            Image::default()
        }
    };
    let has_thumbnail = thumbnail.path().is_some();

    WallpaperCardData {
        title: SharedString::from(title),
        path: SharedString::from(saved_wallpaper.path),
        format: SharedString::from(format),
        thumbnail,
        has_thumbnail,
    }
}

fn wallpaper_title(saved_wallpaper: &SaveWallpaper) -> String {
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
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_uppercase())
        .unwrap_or_else(|| "VIDEO".to_string())
}
