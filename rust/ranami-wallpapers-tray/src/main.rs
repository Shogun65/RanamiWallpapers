use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::PostQuitMessage};
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
use std::ffi::c_void;

use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}};
use tao::event_loop::{ControlFlow, EventLoopBuilder};

use std::env;

// the 0x8001 is test code that print somethink on debug console of
// wallpaper engine after that all numbers are usefull command
// so if you want to test somethink use  WM_ENGINE_TEST and modifie
// _WM_ENGINE_TEST case in windows.cpp to test whatever you want
//
// also if you want to know what what message code Alive wallpaper engine suport
// Check Window.h Window class Private area!
//

use shared::message::*;

fn main() {
    
    let client_data = get_client_data();

    let event_loop = EventLoopBuilder::new().build();

    let tray_menu = Menu::new();

    let test_menu = MenuItem::new("Test", true, None);

    let exit_menu = MenuItem::new("Exit Ranami", true, None);

    let open_gui = MenuItem::new("Change Wallpaper", true, None);

    tray_menu.append(&test_menu).unwrap(); // it not going to give Error belive me
    tray_menu.append(&exit_menu).unwrap();
    tray_menu.append(&open_gui).unwrap();

    let _tray_icon = TrayIconBuilder::new()

    .with_menu(Box::new(tray_menu))

    .with_tooltip("Ranami Wallpapers")
    
    .build()

    .unwrap();

    //let tray_channel = TrayIconEvent::receiver(); not usefull

    let menu_channel = MenuEvent::receiver();


    event_loop.run(move|_event, _window_target, control_flow|
    {
        *control_flow = ControlFlow::Wait; // verry impotand to use "wait"!
 
        if let Ok(event) = menu_channel.try_recv()
        {
            if event.id == test_menu.id()
            {
                prototye_v1(WM_ENGINE_TEST, client_data.client_hwnd);
            }

            if event.id == exit_menu.id()
            {
                prototye_v1(WM_ENGINE_EXIT, client_data.client_hwnd);
                unsafe {
                    PostQuitMessage(0);
                }
            }

            if event.id == open_gui.id()
            {
                prototye_v1(WM_ENGINE_OPEN_GUI, client_data.client_hwnd);
            }

        } 
    });
}

fn prototye_v1(message: u32, hwnd: HWND)
{
    //let hwnd = HWND(0x4074A as *mut c_void);

    unsafe{

        let postmessage_result = PostMessageW(Some(hwnd), 
            message, 
            Default::default(),
            Default::default()
        );

        match postmessage_result {
            Ok(_) =>{ /*println!("No Err on PostMessageW!"); */},
            Err(err) => println!("Err on PostMessageW: {}", err),
        }

    }
}


fn get_client_data() -> ClientData
{
    let args: Vec<String> = env::args().collect();

    let hwnd = usize::from_str_radix(
                                        args.get(1).

                                        expect("No HWND GIVEN!"),

                                         10).unwrap(); // trust me

    /*
        PID is Optional for now!!!!

        why default pid is 4 because pid 4 is for windows it self so we can chack
        if pid is deffent than 4 or not (idk maybe it's a dumb idea?) 
    */

    let pid = match args.get(2) {
        Some(pid) => { pid.parse::<u32>().unwrap_or(4) },
        None => 4,
    };

    let hwnd = HWND(hwnd as *mut c_void);

    return ClientData::new(hwnd, pid);


}


struct ClientData
{
    client_hwnd: HWND,

    #[allow(dead_code)]
    client_pid: u32,

}

impl ClientData {
    fn new(client_hwnd: HWND, client_pid: u32) -> Self
    {
       return ClientData { client_hwnd, client_pid };
    }
}