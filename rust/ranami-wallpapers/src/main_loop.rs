use crate::engine::init_engine::run_wallpaper_engine;
use crate::init_tray::run_tray;
use crate::startup_file_check::{read_file_1, save_file_1};
use crate::{
    window::windows::{ENGINE_HARD_CRASH, ENGINE_HWND},
};
use shared::{namepipe::NamePipeCommands, message::WM_ENGINE_OPEN_GUI, 
    usefull_fn::request_client_post, log_err::err_log};

use windows::Win32::Foundation::{HWND};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, IsZoomed};

use std::process::Child;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn main_loop(
    namepipecommands: Arc<Mutex<NamePipeCommands>>,
    client_hwnd: usize,
) -> Result<(), std::io::Error> {
    let mut current_child: Option<Child> = None;
    let mut current_wallpaper: Option<String> = None;
    let mut ranami_crash = false;
    let mut current_child_tray: Option<Child> = None;
    let mut ranami_crash_count = 0;

    if let Some(wallpaper_path) = read_file_1() {
        if !wallpaper_path.trim().is_empty() {

            let wallpaper_path = std::path::PathBuf::from(wallpaper_path);

            if wallpaper_path.exists(){
                let child = run_wallpaper_engine(
                wallpaper_path.to_str().unwrap_or(""),
                 "3", client_hwnd)?;

                current_child = Some(child);
                current_wallpaper = Some(wallpaper_path.to_string_lossy().to_string());
            }
            else{
                request_client_post(client_hwnd, WM_ENGINE_OPEN_GUI);
            } 
        }
        else {
            request_client_post(client_hwnd, WM_ENGINE_OPEN_GUI);
        }
    }

    'outer: loop {
        if ranami_crash_count > 5 || ENGINE_HARD_CRASH.load(std::sync::atomic::Ordering::Relaxed) {
            set_engine_hwnd_to_0();
            if let Some(mut ranami_core) = current_child.take() {
                let _ = ranami_core.kill();
            }
            if let Some(mut tray_child) = current_child_tray.take() {
                let _ = tray_child.kill();
            }
            break 'outer;
        }

        let mut next_wallpaper: Option<String> = None;

        init_tray(&mut current_child_tray, client_hwnd);
        if check_tray_alive(&mut current_child_tray){break 'outer;}
        

        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if !hwnd.is_invalid() && IsZoomed(hwnd).into() && current_child.is_some()
            {
                kill_ranami_core(&mut current_child);
                // ok you will be confued why this doing here well to use current wallpaper let me explain
                // ranami_crash take current wallpaper and make as next_wallpaper simple enough
                // but why we useing this because it work perfactly fine and also we write less code
                ranami_crash = true;
                continue; // this is improtand
            }
        }

        if ranami_crash {
            if let Some(reuse_wallpaper) = current_wallpaper.take() {
                if !reuse_wallpaper.trim().is_empty(){
                    next_wallpaper = Some(reuse_wallpaper);
                    //current_wallpaper = None; no need for this remember the take()
                }
                else{
                    request_client_post(client_hwnd, WM_ENGINE_OPEN_GUI);
                }
            }
            ranami_crash = false;
        }

        {
            let state = namepipecommands.lock();

            if let Ok(mut state) = state {
                if state.wallpaper_changed {
                    next_wallpaper = Some(state.video_path.clone());
                    state.wallpaper_changed = false;
                }
            }
        }

        if let Some(wallpaper_path) = next_wallpaper {
            if current_wallpaper.as_deref() != Some(wallpaper_path.as_str()) {
                if let Some(mut child) = current_child.take() {
                    let ck = child.kill();
                    println!("core child kill: {:#?}", ck);

                    let cw = child.wait();
                    println!("core child wait: {:#?}", cw);
                    kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);
                }

                let child = run_wallpaper_engine(&wallpaper_path, "3", client_hwnd)?;

                let save_file_1_result = save_file_1(
                    std::path::PathBuf::from(&wallpaper_path));

                if let Err(err) = save_file_1_result {
                    err_log(&format!("err on save_file_1 on main_loop: {}", err));
                }
                current_child = Some(child);
                current_wallpaper = Some(wallpaper_path);
            }
        }

        if let Some(mut_child) = current_child.as_mut() {
            if let Ok(Some(exit_status)) = mut_child.try_wait() {
                println!("Exit status : {}", exit_status);
                current_child = None;

                if exit_status.success() {
                    println!("success exit code 0");
                    // NOTE: tray when click exit on tray it will exit him self but this code is kinda useles
                    // but it here because maybe if somehow "Ranami core" exit whit exit code 0 so we also kill him(tray)
                    kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);
                    break 'outer;
                };

                ranami_crash = true;
                ranami_crash_count += 1;
                kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray);
            }
        }

        if check_tray_alive(&mut current_child_tray){break 'outer;}
        thread::sleep(Duration::from_millis(10));
    }
    kill_tray_and_set_engine_hwnd_to_0(&mut current_child_tray); // just in case
    return Ok(());
}

fn get_engine_hwnd() -> usize {
    return ENGINE_HWND.load(std::sync::atomic::Ordering::Relaxed);
}

fn set_engine_hwnd_to_0() {
    ENGINE_HWND.store(0, std::sync::atomic::Ordering::Release);
}

fn init_tray(current_child_tray: &mut Option<Child>, client_hwnd: usize) {
    if current_child_tray.is_some() {
        return;
    } //
    println!("Engine HWND: {} (init_tray)", get_engine_hwnd());

    let result_of_tray = run_tray(client_hwnd);
    if let Ok(child) = result_of_tray {
        *current_child_tray = Some(child);
    } else {
        err_log(&format!(
            "Error on init_tray() on main_loop: {}",
            result_of_tray.unwrap_err()
        ));
    }
}

fn kill_tray_and_set_engine_hwnd_to_0(tray_child: &mut Option<Child>) {
    set_engine_hwnd_to_0(); // verry improtand
    if let Some(mut tray_child) = tray_child.take() {
        let ck = tray_child.kill();
        println!("tray child kill: {:?}", ck);

        let cw = tray_child.wait();
        println!("tray child wait: {:?}", cw);
    }
}

fn kill_ranami_core(current_child : &mut Option<Child>){
    if let Some(mut ranami_child) = current_child.take(){
        let rt = ranami_child.kill();
        
        println!("kill_ranami_core: {:?}", rt);
        if let Err(err) = rt{
            err_log(&format!("error on killing the child on kill_ranami_core: {err}"));
        }
    }
}

fn check_tray_alive(current_child_tray: &mut Option<Child>) -> bool {

    if let Some(tray_child) = current_child_tray.as_mut() {

        if let Ok(Some(exit_status)) = tray_child.try_wait() {

            println!("tray exit status: {}", exit_status);
            if exit_status.success() {
                *current_child_tray = None;
                return true;
            }
            else {
                *current_child_tray = None; // make the None so on top init_tray can run
                // tray again thats it
                return false;
            }
        }
    };
    return false;
}