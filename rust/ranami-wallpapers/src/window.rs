use std::os::raw::c_void;

use ::windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::PostMessageW};

use crate::{client_init::log_err::err_log, window::windows::ENGINE_HWND};

pub(crate) mod windows{
    use std::{os::raw::c_void, process::Child, thread::{self, JoinHandle}};
    use windows::Win32::Foundation::*; // idc man i want to code not to do this all day
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::w;
    use crate::{init_gui::init_gui::run_gui, window::postmessage};

    use super::super::client_init::log_err::err_log;
    use std::sync::mpsc::{channel, Sender};
    use shared::message::*; // take all message
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static ENGINE_HWND : AtomicUsize = AtomicUsize::new(0);
    static mut GUI_CHILD: Option<Child> = None; // trust me man

    fn run_main_window(tx: Sender<usize>) -> JoinHandle<()>
    {

        let handle = thread::spawn(move || {

            unsafe{

                let class_name = w!("HiddenWindow");
            
                let wc = WNDCLASSW{
                    lpfnWndProc : Some(window_proc),
                    lpszClassName : class_name,
                    ..Default::default()
                };

                let idk = RegisterClassW(&wc);

                if idk == 0 {
                    err_log("Failed to register window class");
                    return;
                }

                let hwnd = CreateWindowExW(
                                Default::default(), 
                                    class_name, 
                                    w!(""), 
                                    WS_POPUP,
                                     0, 0, 0, 0,
                                      None, None, None, None).unwrap();

                match tx.send(hwnd.0 as usize) {
                    Ok(_) => {},
                    Err(err) => err_log(&format!("Cant sent the tx of HWND: {}", err)),
                }


                let mut msg = MSG::default();

                while GetMessageW(&mut msg, None, 0, 0).into()
                {
                    // let _ = TranslateMessage(&mut msg);
                    DispatchMessageW(&mut msg);
                }
            }
        });

        return handle;
    }

    
    extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,) -> LRESULT 
    {
        unsafe {

            match msg {

                WM_DESTROY =>{
                    PostQuitMessage(0);
                    return LRESULT(0);
                }

                WM_ENGINE_SENT_HWND =>{

                    let hwnd = HWND(lparam.0 as *mut c_void);
                    println!("hwnd of engine: {:?}", hwnd);
                    println!("hwnd of engine(usize): {:?}", hwnd.0 as usize);

                    ENGINE_HWND.store(hwnd.0 as usize, Ordering::Release);

                    return LRESULT(0);
                }

                WM_ENGINE_TEST =>{
                    println!("Get WM_ENGINE_TEST from tray");
                    return LRESULT(0);
                }

                WM_ENGINE_EXIT =>{
                    postmessage(WM_ENGINE_EXIT);
                    let gui = &raw mut GUI_CHILD;
                    if let Some(mut child) = (*gui).take(){
                        let _ = child.kill();
                    }
                    PostQuitMessage(0);
                    return LRESULT(0);
                }

                WM_ENGINE_OPEN_GUI =>{

                    let gui = &raw mut GUI_CHILD;
                    
                    if let Some(mut child) = (*gui).take(){

                        if let Ok(exit_status) =  child.try_wait(){

                            match exit_status {

                                Some(exit_status) =>{
                                    
                                    let rt = run_gui();
                                    if let Ok(child) = rt{
                                        (*gui) = Some(child);
                                    }
                                    else{
                                        let rt = rt.unwrap_err();
                                        err_log(&format!("err on GUI_CHILD: {},
                                                                 old exit status: {}", rt, exit_status));
                                    }
                                    return LRESULT(0);
                                },

                                None => {
                                    (*gui) = Some(child); // we just put that child back because he still
                                    return LRESULT(0); // runing
                                },
                            }
                        }
                        else{
                            (*gui) = Some(child);
                            err_log("Error on try_wait on GUI_CHILD");
                        }
                    }
                    else{
                        let rt = run_gui();

                        match rt {
                            Ok(child) => (*gui) = Some(child),
                            Err(err) => err_log(&format!("Error on lunching the run_gui: {}", err)),
                        }

                    }
                    return LRESULT(0);
                }

                _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
            }
            
        }
    }

    pub(crate) fn init_window() -> InitWindowData
    {
        let (tx, rx) = channel::<usize>();

        let handle = run_main_window(tx);

        match rx.recv() {
            Ok(hwnd) => { 
                return InitWindowData::new(handle, Some(hwnd)); 
            },

            Err(err) => {
                err_log(&format!("Cant sent the tx of HWND: {}", err)); 
                return InitWindowData::new(handle, None);
            },
        };
    }

    pub struct InitWindowData{
        pub handle : JoinHandle<()>,
        pub main_hwnd : Option<usize>
    }

    impl InitWindowData {
        pub(crate) fn new(handle : JoinHandle<()>, main_hwnd : Option<usize>) -> Self
        {
            return InitWindowData { handle, main_hwnd };
        }
    }

}

fn postmessage(message: u32){
    unsafe {
        let hwnd = ENGINE_HWND.load(std::sync::atomic::Ordering::Relaxed);

        if hwnd == 0 {return;}

        let hwnd = HWND(hwnd as *mut c_void);

        let rt = PostMessageW(
            Some(hwnd),
            message,
            Default::default(),
            Default::default()
        );
        if let Err(err) = rt{
            err_log(&format!("Err on postmessage on windows.rs: {}", err));
        }
    }
}