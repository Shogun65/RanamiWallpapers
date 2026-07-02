
pub(crate) mod file_saver{

    use std::fs::{self, OpenOptions};
    use std::io::{Write, Read};
    use std::path::{Path, PathBuf};
    use rfd::FileHandle;
    use shared::save_wallpaper::SaveWallpaper;

    #[allow(dead_code)]
    pub(crate) fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error>{

        let mut path_file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .open("Saved-Wallpaper-Path.txt")?;

        write!(path_file, "{}", video_path.to_str().unwrap_or("None (unwrap_or failed!)"))?;

        return Ok(());
    }

    #[allow(dead_code)]
    pub fn read_file_1() -> Option<String> {


        let mut video_path_file = OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open("Saved-Wallpaper-Path.txt").ok()?;

        let mut video_path = String::new();
        
        video_path_file.read_to_string(&mut video_path).ok()?;

        return Some(video_path);
    }

    /*

        New read and save version here that use json

    */

    pub fn save_file_2(file_handle: FileHandle) -> Result<(), std::io::Error>{
        
        let video_path = file_handle.path()
                                .to_str().unwrap_or("Error Path")
                                .to_string();

        let save_wallpaper = SaveWallpaper{
            name : "nan".to_string(),
            path : video_path
        };
        
        let mut vec_wallpaper: Vec<SaveWallpaper> = if Path::new("Save-Wallpapers.json").exists(){

            let json = std::fs::read_to_string("Save-Wallpapers.json")?;

            serde_json::from_str(&json).unwrap_or(Vec::new())
        }
        else {
            Vec::new()
        };

        vec_wallpaper.push(save_wallpaper);

        let json = serde_json::to_string_pretty(&vec_wallpaper)?;

        fs::write("Save-Wallpapers.json", json)?;

        return Ok(());

    }

}