pub(crate) mod init_engine 
{
    use std::process::{Command, Child};

    // first video_path than buffer count and than client hwnd that we senting as usize (check Parse.cpp for mroe info i guess?)
    pub(crate) fn run_wallpaper_engine(
        video_path: &str, 
        buffer_count: &str, 
        client_hwnd: usize) -> Result<Child, std::io::Error>
    {

        let process = Command::new("RanamiWallpapers.exe")
        .arg(video_path)
        .arg(buffer_count)
        .arg(client_hwnd.to_string())
        .spawn()?;

        return Ok(process);
    }
}


// r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4"