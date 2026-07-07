//#![windows_subsystem = "windows"]

mod client_init;
mod engine;
mod window;
mod arg;
mod init_tray;
mod init_gui;
mod client_loop;
mod namepipe;

use client_init::client_init::client_init;
use client_init::error::ErrorClient;
use client_init::log_err::err_log;
use window::windows::init_window;
use arg::{init, error};
use shared::namepipe::{NamePipeCommands};
use std::sync::{Arc, Mutex};
use client_loop::init_loop;

fn main() 
{

    // that init fun for arg parse.. well sometime nameing get messy but anyways
    match init::init() {

        Ok(_) => {},

        Err(error::ErrorArg::ConsoleErr(err)) => {
            err_log(&format!("Console Error: {}", err)); return;},
    }
    
    let namepipecommands: Arc<Mutex<NamePipeCommands>> = Arc::new(
                                                  Mutex::new(
                                                        NamePipeCommands{
                                                            video_path : "NONE".to_string(), 
                                                            wallpaper_changed : false
                                                        }));

    let client_result = client_init();

    match client_result {
        Ok(_) => {},
        Err(ErrorClient::MissingFile(file_name)) => {
            err_log(&format!("MissingFile: {}", file_name)); return;},
    };

    let init_window_data = init_window();

    let _window_handle = init_window_data.handle;

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

    let _ = init_loop::run(client_hwnd, namepipecommands);
}


