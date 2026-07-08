

use std::sync::{Arc, Mutex};
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
    let mut currant_hwnd_of_ranami_core: usize = 0;

    loop {
        
        let mut next_wallpaper: Option<String> = None;

        {
            let mut state = namepipecommands.lock();

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
                    let _ = child.kill();// maybe someday i handle this error
                    let _ = child.wait();
                }
            
                let child = run_wallpaper_engine(
                    &wallpaper_path, "3", client_hwnd)?;
            
                current_child = Some(child);
                current_wallpaper = Some(wallpaper_path);
            }
        }





    }






    return Ok(());
}