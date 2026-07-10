pub(crate) mod windows{
    use std::{os::raw::c_void, thread::{self, JoinHandle}};
    use windows::Win32::Foundation::*; // idc man i want to code not to do this all day
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::w;
    use super::super::client_init::log_err::err_log;
    use std::sync::mpsc::{channel, Sender};
    use shared::message::*; // take all message
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static ENGINE_HWND : AtomicUsize = AtomicUsize::new(0); 

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