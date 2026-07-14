use shared::save_path_and_settings::{legacy_startup_file_path, startup_file_path};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error> {
    let startup_path = startup_file_path()?;

    let mut path_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(startup_path)?;

    write!(
        path_file,
        "{}",
        video_path.to_str().unwrap_or("None (unwrap_or failed!)")
    )?;

    return Ok(());
}

pub fn read_file_1() -> Option<String> {
    let startup_path = ensure_startup_file_path().ok()?;

    let mut video_path_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(startup_path)
        .ok()?;

    let mut video_path = String::new();

    video_path_file.read_to_string(&mut video_path).ok()?;

    return Some(video_path);
}

fn ensure_startup_file_path() -> Result<PathBuf, std::io::Error> {
    let startup_path = startup_file_path()?;

    if startup_path.exists() {
        return Ok(startup_path);
    }

    let legacy_path = legacy_startup_file_path()?;

    // Copy the old repo-local startup file once so the current wallpaper survives the storage move.
    if legacy_path != startup_path && legacy_path.exists() {
        fs::copy(&legacy_path, &startup_path)?;
    }

    Ok(startup_path)
}
