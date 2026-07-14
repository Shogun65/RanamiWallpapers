pub(crate) mod file_saver {
    use rfd::FileHandle;
    use shared::save_path_and_settings::*;
    use shared::save_wallpaper::SaveWallpaper;
    use std::ffi::OsStr;
    use std::fs::{self, OpenOptions};
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    // The GUI keeps its wallpaper library in LocalAppData so it survives regardless of launch folder.

    #[allow(dead_code)]
    pub(crate) fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error> {
        // Legacy single-path storage kept for reference while the app now uses json lists.
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
        // Matches save_file_1 above: older experiments read just one saved wallpaper path.
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

    pub fn read_saved_wallpapers() -> Result<Vec<SaveWallpaper>, std::io::Error> {
        let saved_wallpapers_path = ensure_saved_wallpapers_path()?;

        // Missing json simply means the user has not imported anything yet.
        if !saved_wallpapers_path.exists() {
            return Ok(Vec::new());
        }

        let json = fs::read_to_string(saved_wallpapers_path)?;

        // Invalid json should not crash the GUI; treat it like an empty library for now.
        Ok(serde_json::from_str(&json).unwrap_or_default())
    }

    pub fn read_existing_saved_wallpapers() -> Result<Vec<SaveWallpaper>, std::io::Error> {
        // Drop saved entries whose video file no longer exists and sync the json file to match.
        let mut saved_wallpapers = read_saved_wallpapers()?;
        let original_len = saved_wallpapers.len();

        saved_wallpapers.retain(|wallpaper| Path::new(&wallpaper.path).exists());

        if saved_wallpapers.len() != original_len {
            write_saved_wallpapers(&saved_wallpapers)?;
        }

        Ok(saved_wallpapers)
    }

    pub fn save_file_2(file_handle: FileHandle) -> Result<(), std::io::Error> {
        // Save both the original path and a friendly card title derived from the file name.
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

        // Importing the same path twice should refresh its saved name instead of duplicating the card.
        if let Some(existing_wallpaper) = vec_wallpaper
            .iter_mut()
            .find(|wallpaper| wallpaper.path == save_wallpaper.path)
        {
            existing_wallpaper.name = save_wallpaper.name;
        } else {
            vec_wallpaper.push(save_wallpaper);
        }

        let json = serde_json::to_string_pretty(&vec_wallpaper)?;

        fs::write(save_wallpapers_file_path()?, json)?;

        return Ok(());
    }

    fn write_saved_wallpapers(saved_wallpapers: &[SaveWallpaper]) -> Result<(), std::io::Error> {
        // Shared helper so cleanup and import flows both write json the same way.
        let json = serde_json::to_string_pretty(saved_wallpapers)?;

        fs::write(save_wallpapers_file_path()?, json)?;

        Ok(())
    }

    fn ensure_saved_wallpapers_path() -> Result<PathBuf, std::io::Error> {
        let saved_wallpapers_path = save_wallpapers_file_path()?;

        if saved_wallpapers_path.exists() {
            return Ok(saved_wallpapers_path);
        }

        let legacy_path = legacy_save_wallpapers_file_path()?;

        // Copy the old repo-local json once so existing imports keep showing up after the move.
        if legacy_path != saved_wallpapers_path && legacy_path.exists() {
            fs::copy(&legacy_path, &saved_wallpapers_path)?;
        }

        Ok(saved_wallpapers_path)
    }
}
