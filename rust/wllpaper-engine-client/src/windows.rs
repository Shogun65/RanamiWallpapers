pub(crate) mod windows{
    use std::thread;
    use windows::Win32::Foundation::*; // idc man i want to code not to do this all day
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::w;
    use super::super::client_init::log_err::err_log;
    use std::sync::mpsc;

    fn run_main_window()
    {

        let _handle = thread::spawn(||{

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
                                     0, 0, 0, 0, None, None, None, None);


                let mut msg = MSG::default();

                while GetMessageW(&mut msg, None, 0, 0).into()
                {
                    // let _ = TranslateMessage(&mut msg);
                    DispatchMessageW(&mut msg);
                }
            }
        });
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

    pub(crate) fn init_window() -> HWND
    {
        
    }
}