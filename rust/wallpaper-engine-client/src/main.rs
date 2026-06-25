#![windows_subsystem = "windows"]

mod client_init;
mod engine;
mod window;
mod arg;
mod init_tray;

use client_init::client_init::client_init;
use client_init::error::ErrorClient;
use engine::init_engine::run_wallpaper_engine;
use client_init::log_err::err_log;
use window::windows::init_window;
use arg::{init, error};

fn main() {

    // that init fun for arg parse.. well sometime nameing get messy but anyways
    match init::init() {
        Ok(_) => {},
        Err(error::Error::ConsoleErr(err)) => {
            err_log(&format!("Console Error: {}", err)); return;},
    }
    
    let client_result = client_init();

    match client_result {
        Ok(_) => {},
        Err(ErrorClient::MissingFile(file_name)) => {
            err_log(&format!("MissingFile: {}", file_name)); return;},
    };

    let init_window_data = init_window();

    let handle = init_window_data.handle;

    let client_hwnd = init_window_data.main_hwnd; // iknow main hwnd and client hand geting mess but they both same thing hehe

    let client_hwnd = match client_hwnd {

        Some(hwnd) => {
            println!("client_hwnd: {}", hwnd); 
            hwnd
        },

        None => {
            err_log("main_hwnd is None!!!");
            return;
        },
    };

    let video = r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4";

    let engine_process = match run_wallpaper_engine(video, "3", client_hwnd) {
        Ok(process) => process,
        
        Err(err) => {
            eprintln!("Error: {}", err); 
            return;
        },
    };

    println!("engine_process: {}", engine_process.id());

    match handle.join() {
        Ok(_) => { println!("Window thread exit normaly"); },
        Err(err) => {
            err_log(&format!("Window thread panic: {:?}", err));
            return;
        }
    }

}


