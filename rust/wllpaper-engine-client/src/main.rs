mod client_init;
mod engine;
mod windows;

use client_init::client_init::client_init;
use client_init::error::ErrorClient;
use engine::init_engine::run_wallpaper_engine;
use client_init::log_err::err_log;
use windows::windows::init_window;

fn main() {
    
    // let client_result = client_init();

    // match client_result {
    //     Ok(_) => {},
    //     Err(ErrorClient::MissingFile(file_name)) => {
    //         err_log(&format!("MissingFile: {}", file_name)); return;},
    // };

    let main_window = init_window();

    let handle = main_window.handle;

    let main_hwnd = main_window.main_hwnd;

    match main_hwnd {
        Some(hwnd) => println!("main_hwnd: {:?}", hwnd),
        None => {
            err_log("main_hwnd is None!!!");
            return;
        },
    }

    match handle.join() {
        Ok(_) => { println!("Window thread exit normaly"); },
        Err(err) => {
            err_log(&format!("Window thread panic: {:?}", err));
            return;
        }
    }

    // let video = r"C:\Users\gmy87\Downloads\ayanami-rei-beneath-blue-light.3840x2160.mp4";

    // let engine_process = match run_wallpaper_engine(video, "3") {
    //     Ok(process) => process,
    //     Err(err) => {eprintln!("Error: {}", err); return;},
    // };

    // println!("engine_process: {}", engine_process.id());

}


