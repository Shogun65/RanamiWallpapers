pub(crate) mod windows{
    use std::thread::{self, JoinHandle};
    use windows::Win32::Foundation::*; // idc man i want to code not to do this all day
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::w;
    use super::super::client_init::log_err::err_log;
    use std::sync::mpsc::{channel, Sender};
    use std::ffi::c_void;
    //use windows_result::Error;

    fn run_main_window(tx: Sender<isize>) -> JoinHandle<()>
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

                match tx.send(hwnd.0 as isize) {
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

                _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
            }
            
        }
    }

    pub(crate) fn init_window() -> InitWindowData
    {
        let (tx, rx) = channel::<isize>();

        let handle = run_main_window(tx);

        match rx.recv() {
            Ok(hwnd) => { 
                let main_hwnd = HWND(hwnd as *mut c_void);
                return InitWindowData::new(handle, Some(main_hwnd)); 
            },

            Err(err) => {
                err_log(&format!("Cant sent the tx of HWND: {}", err)); 
                return InitWindowData::new(handle, None);
            },
        };
    }

    pub struct InitWindowData{
        pub handle : JoinHandle<()>,
        pub main_hwnd : Option<HWND>
    }

    impl InitWindowData {
        pub(crate) fn new(handle : JoinHandle<()>, main_hwnd : Option<HWND>) -> Self
        {
            return InitWindowData { handle, main_hwnd };
        }
    }

}