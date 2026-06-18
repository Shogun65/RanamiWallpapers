mod client_init;
mod engine;

use client_init::client_init::client_init;
use client_init::error::Error;
use engine::init_engine::run_wallpaper_engine;
use crate::client_init::log_err::log;

fn main() {
    
    let client_result = client_init();

    let log_rusult = match client_result {
        Ok(_) => Ok(()),
        Err(Error::CantOpenDebugFile) => log("CantOpenDebugFile"),
        Err(Error::CantWriteDebugError) => log("CantWriteDebugError"),
        Err(Error::MissingFile(file_name)) => log(&format!("MissingFile: {}", file_name)),
    };

    match log_rusult {
        Ok(_) => {/* idk man do nothink */},
        Err(Error::CantOpenDebugFile) => panic!("CantOpenDebugFile"),
        Err(Error::CantWriteDebugError) => panic!("CantWriteDebugError"),
        Err(Error::MissingFile(file_name)) => panic!("MissingFile: {}", file_name),
    }

    let video = r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4";

    let engine_process = match run_wallpaper_engine(video, "3") {
        Ok(process) => process,
        Err(err) => {eprintln!("Error: {}", err); return;},
    };

    println!("engine_process: {}", engine_process.id());

}


