

use std::{sync::{Arc, Mutex}, thread, time::Duration};
use shared::namepipe::NamePipeCommands;
use crate::window::windows::ENGINE_HWND;
use std::process::Child;
use crate::engine::init_engine::run_wallpaper_engine;

pub fn main_loop(

    namepipecommands : Arc<Mutex<NamePipeCommands>>,
    client_hwnd : usize


) -> Result<(), std::io::Error>
{
    let mut current_child: Option<Child> = None;
    let mut current_wallpaper: Option<String> = None;
    let mut ranami_crash = false;
    let mut currant_hwnd_of_ranami_core: usize = 0;

    loop {
        
        let mut next_wallpaper: Option<String> = None;

        if ranami_crash{
            if let Some(reuse_wallpaper) = current_wallpaper.as_deref(){
                next_wallpaper = Some(reuse_wallpaper.to_string());
                current_wallpaper = None;
            }
            ranami_crash = false;
        }

        {
            let state = namepipecommands.lock();

            if let Ok(mut state) = state{
                if state.wallpaper_changed{
                    next_wallpaper = Some(state.video_path.clone());
                    state.wallpaper_changed = false;
                }
            }

        }

        if let Some(wallpaper_path) = next_wallpaper{
            if current_wallpaper.as_deref() != Some(wallpaper_path.as_str()){
                if let Some(mut child) = current_child.take(){

                    let ck = child.kill();
                    println!("child kill: {:#?}", ck);

                    let cw = child.wait();
                    println!("child wait: {:#?}", cw);
                }
            
                let child = run_wallpaper_engine(
                    &wallpaper_path, "3", client_hwnd)?;
            
                current_child = Some(child);
                current_wallpaper = Some(wallpaper_path);
            }
        }

        if let Some(mut_child) = current_child.as_mut(){
            if let Ok(Some(exit_status)) = mut_child.try_wait(){
                println!("Exit status : {}", exit_status);
                current_child = None;
                ranami_crash = true;
                if exit_status.success(){
                    println!("success exit code 0");
                    break;
                };

                //current_wallpaper = None;
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
    return Ok(());
}

fn get_engine_hwnd() -> usize{
    return ENGINE_HWND.load(std::sync::atomic::Ordering::Relaxed);
}