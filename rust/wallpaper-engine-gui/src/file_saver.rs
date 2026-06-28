

pub(crate) mod file_saver{

    use std::fs::OpenOptions;
    use std::io::{Write, Read};
    use std::path::PathBuf;

    pub(crate) fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error>{

        let mut path_file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .open("Saved-Wallpaper-Path.txt")?;

        write!(path_file, "{}", video_path.to_str().unwrap_or("None (unwrap_or failed!)"))?;

        return Ok(());
    }

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



}