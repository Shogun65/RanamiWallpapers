

pub(crate) mod file_saver{

    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::PathBuf;

    pub(crate) fn save_file_1(video_path: PathBuf) -> Result<(), std::io::Error>{

        let mut path_file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .open("Saved-Wallpaper-Path.txt")?;

        write!(path_file, "{}", video_path.to_str().unwrap_or("None (unwrap_or failed!)"))?;

        return Ok(());
    }



}