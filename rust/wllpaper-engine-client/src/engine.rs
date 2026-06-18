pub(crate) mod init_engine 
{
    use std::process::{Command, Child};

    pub(crate) fn run_wallpaper_engine(video_path: &str, buffer_count: &str) -> Result<Child, std::io::Error>
    {

        let process = Command::new("AliveWallpaperEngine.exe")
        .arg(video_path)
        .arg(buffer_count)
        .spawn()?;

        return Ok(process);
    }
}


// r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4"