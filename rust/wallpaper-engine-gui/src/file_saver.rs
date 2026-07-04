pub(crate) mod file_saver {

    use rfd::FileHandle;
    use shared::save_wallpaper::SaveWallpaper;
    use std::ffi::OsStr;
    use std::fs::{self, OpenOptions};
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    const SAVE_WALLPAPERS_PATH: &str = "Save-Wallpapers.json";

    #[allow(dead_code)]
    pub(crate) fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error> {
        let mut path_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("Saved-Wallpaper-Path.txt")?;

        write!(
            path_file,
            "{}",
            video_path.to_str().unwrap_or("None (unwrap_or failed!)")
        )?;

        return Ok(());
    }

    #[allow(dead_code)]
    pub fn read_file_1() -> Option<String> {
        let mut video_path_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("Saved-Wallpaper-Path.txt")
            .ok()?;

        let mut video_path = String::new();

        video_path_file.read_to_string(&mut video_path).ok()?;

        return Some(video_path);
    }

    /*

        New read and save version here that use json

    */

    pub fn read_saved_wallpapers() -> Result<Vec<SaveWallpaper>, std::io::Error> {
        if !Path::new(SAVE_WALLPAPERS_PATH).exists() {
            return Ok(Vec::new());
        }

        let json = fs::read_to_string(SAVE_WALLPAPERS_PATH)?;

        Ok(serde_json::from_str(&json).unwrap_or_default())
    }

    pub fn save_file_2(file_handle: FileHandle) -> Result<(), std::io::Error> {
        let video_path = file_handle.path().to_string_lossy().to_string();

        let wallpaper_name = file_handle
            .path()
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or("Untitled wallpaper")
            .to_string();

        let save_wallpaper = SaveWallpaper {
            name: wallpaper_name,
            path: video_path,
        };

        let mut vec_wallpaper = read_saved_wallpapers()?;

        if let Some(existing_wallpaper) = vec_wallpaper
            .iter_mut()
            .find(|wallpaper| wallpaper.path == save_wallpaper.path)
        {
            existing_wallpaper.name = save_wallpaper.name;
        } else {
            vec_wallpaper.push(save_wallpaper);
        }

        let json = serde_json::to_string_pretty(&vec_wallpaper)?;

        fs::write(SAVE_WALLPAPERS_PATH, json)?;

        return Ok(());
    }
}
