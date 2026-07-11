//#![windows_subsystem = "windows"]

mod client_init;
mod engine;
mod window;
mod arg;
mod init_tray;
mod init_gui;
mod namepipe;
mod main_loop;

use client_init::client_init::client_init;
use client_init::error::ErrorClient;
use client_init::log_err::err_log;
use window::windows::{init_window};
use arg::{init, error};
use shared::namepipe::{NamePipeCommands};
use std::sync::{Arc, Mutex};
use crate::namepipe::init_namepipe::run_namepipe_server;

use tokio::runtime::{Handle, Runtime};



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

    let _ = init_window_data.handle;

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

    let runtime_data = match RuntimeAndHandle::new() {
        Some(rt) => rt,
        None => return,
    };

    let _ = run_namepipe_server(
        namepipecommands.clone(),
        &runtime_data.handle,
    );

    println!("running the main loop");
    let rt = main_loop::main_loop(namepipecommands.clone(), client_hwnd);

    if let Err(err) = rt{
        err_log(&format!("err on main loop: {}", err));
    }

}


struct RuntimeAndHandle{
    
    handle : Handle,

    #[allow(dead_code)]
    runtime : Runtime // poor runtime too bad we just neeed him for his handle soo sad maybe in future we need you
}


impl RuntimeAndHandle {
    fn new() -> Option<Self>{
        
        let runtime = Runtime::new();

        if let Ok(runtime) = runtime{
            let handle = runtime.handle().clone();
            return Some(RuntimeAndHandle { runtime, handle });
        };

        err_log(&format!("error on RuntimeAndHandle new(): {}", runtime.unwrap_err()));
        return None;
    }
}