use shared::save_path_and_settings::STARTUP_FILE_SAVE_NAME;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error> {
    let mut path_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(STARTUP_FILE_SAVE_NAME)?;

    write!(
        path_file,
        "{}",
        video_path.to_str().unwrap_or("None (unwrap_or failed!)")
    )?;

    return Ok(());
}

pub fn read_file_1() -> Option<String> {
    let mut video_path_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(STARTUP_FILE_SAVE_NAME)
        .ok()?;

    let mut video_path = String::new();

    video_path_file.read_to_string(&mut video_path).ok()?;

    return Some(video_path);
}
