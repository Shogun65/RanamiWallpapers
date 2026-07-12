
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use shared::namepipe::NamePipeCommands;
use crate::{client_init::log_err::err_log, window::windows::ENGINE_HWND};
use std::process::Child;
use crate::engine::init_engine::run_wallpaper_engine;
use crate::init_tray::run_tray;
use crate::startup_file_check::{read_file_1, save_file_1};

pub fn main_loop(

    namepipecommands : Arc<Mutex<NamePipeCommands>>,
    client_hwnd : usize

) -> Result<(), std::io::Error>
{
    let mut current_child: Option<Child> = None;
    let mut current_wallpaper: Option<String> = None;
    let mut ranami_crash = false;
    let mut current_child_tray: Option<Child> = None;

    if let Some(wallpaper_path) = read_file_1(){
        let child = run_wallpaper_engine(
                &wallpaper_path, "3", client_hwnd)?;
            
        current_child = Some(child);
        current_wallpaper = Some(wallpaper_path);  
    }

    'outer: loop {
        
        let mut next_wallpaper: Option<String> = None;

        init_tray(&mut current_child_tray, client_hwnd);

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
                    println!("core child kill: {:#?}", ck);

                    let cw = child.wait();
                    println!("core child wait: {:#?}", cw);
                    kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);
                }
            
                let child = run_wallpaper_engine(
                    &wallpaper_path, "3", client_hwnd)?;
                
                let save_file_1_result = save_file_1(
                                                std::path::PathBuf::from(&wallpaper_path));
                
                if let Err(err) = save_file_1_result{
                    err_log(&format!("err on save_file_1 on main_loop: {}", err));
                }
                current_child = Some(child);
                current_wallpaper = Some(wallpaper_path);
            }
        }

        if let Some(mut_child) = current_child.as_mut(){
            if let Ok(Some(exit_status)) = mut_child.try_wait(){
                println!("Exit status : {}", exit_status);
                current_child = None;
                
                if exit_status.success(){
                    println!("success exit code 0");
                    // NOTE: tray when click exit on tray it will exit him self but this code is kinda useles
                    // but it here because maybe if somehow "Ranami core" exit whit exit code 0 so we also kill him(tray)
                    kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);
                    break 'outer;
                };

                ranami_crash = true;
                kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);
            }
        }

        if let Some(tray_child) = current_child_tray.as_mut(){
            if let Ok(Some(exit_status)) = tray_child.try_wait()  {
                println!("tray exit status: {}", exit_status);
                if exit_status.success(){
                    current_child_tray.take();
                    break 'outer;
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);// just in case
    return Ok(());
}

fn get_engine_hwnd() -> usize{
    return ENGINE_HWND.load(std::sync::atomic::Ordering::Relaxed);
}

fn set_engine_hwnd_to_0(){
    ENGINE_HWND.store(0, std::sync::atomic::Ordering::Release);
}

fn init_tray(
    current_child_tray: &mut Option<Child>, 
    client_hwnd : usize,
)
{
    
    if current_child_tray.is_some() {return;}//
    println!("Engine HWND: {} (init_tray)", get_engine_hwnd());
    
    let result_of_tray = run_tray(client_hwnd);
    if let Ok(child) = result_of_tray{
        *current_child_tray = Some(child);
    }
    else{
        err_log(&format!("Error on init_tray() on main_loop: {}", 
        result_of_tray.unwrap_err()));
    }
}

fn kill_tray_and_set_engine_hwnd_to_0(tray_child : &mut Option<Child>){

    set_engine_hwnd_to_0(); // verry improtand
    if let Some(mut tray_child) = tray_child.take(){
                    
        let ck = tray_child.kill();
        println!("tray child kill: {:?}", ck);

        let cw = tray_child.wait();
        println!("tray child wait: {:?}", cw);

    }
}