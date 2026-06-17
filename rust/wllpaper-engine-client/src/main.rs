mod client_init;

use std::{process::{Child, Command}};

fn main() {
    
    let engine_process = match run_wallpaper_engine() {
        Ok(process) => process,
        Err(err) => {eprintln!("Error: {}", err); return;},
    };

    println!("engine_process: {}", engine_process.id());

}


fn run_wallpaper_engine() -> Result<Child, std::io::Error>
{
    let process = Command::new("AliveWallpaperEngine.exe")
    .arg(r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4")
    .arg("3")
    .spawn()?;

    return Ok(process);
}