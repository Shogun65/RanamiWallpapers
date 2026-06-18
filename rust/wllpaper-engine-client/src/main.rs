mod client_init;
mod engine;

use client_init::client_init::client_init;
use client_init::error::ErrorClient;
use engine::init_engine::run_wallpaper_engine;
use crate::client_init::log_err::err_log;

fn main() {
    
    let client_result = client_init();

    match client_result {
        Ok(_) => {},
        Err(ErrorClient::MissingFile(file_name)) => {
            err_log(&format!("MissingFile: {}", file_name)); return;},
    };

    let video = r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4";

    let engine_process = match run_wallpaper_engine(video, "3") {
        Ok(process) => process,
        Err(err) => {eprintln!("Error: {}", err); return;},
    };

    println!("engine_process: {}", engine_process.id());

}


