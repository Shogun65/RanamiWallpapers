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
use engine::init_engine::run_wallpaper_engine;
use client_init::log_err::err_log;
use window::windows::init_window;
use arg::{init, error};
use shared::namepipe::{NamePipeCommands, PIPE_NAME};
use core::time;
use std::{sync::{Arc, Mutex}, thread};
use namepipe::init_namepipe;
use tokio::runtime::Runtime;

fn main() {

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

    // let client_result = client_init();

    // match client_result {
    //     Ok(_) => {},
    //     Err(ErrorClient::MissingFile(file_name)) => {
    //         err_log(&format!("MissingFile: {}", file_name)); return;},
    // };

    // let init_window_data = init_window();

    // let window_handle = init_window_data.handle;

    // let client_hwnd = init_window_data.main_hwnd; // iknow main hwnd and client hand geting mess but they both same thing hehe

    // let client_hwnd = match client_hwnd {

    //     Some(hwnd) => {
    //         println!("client_hwnd: {}", hwnd); 
    //         hwnd
    //     },

    //     None => {
    //         err_log("main_hwnd is None!!!");
    //         return;
    //     },
    // };


    let runtime = Runtime::new().unwrap();

    let runtime_handle = runtime.handle();

    let _namepipe_handle = init_namepipe::run_namepipe_server(namepipecommands.clone(), runtime_handle);

    loop {
        thread::sleep(time::Duration::from_secs(1));
        println!("{:#?}", namepipecommands);
    }

}


