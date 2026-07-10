use std::process::{Child, Command};


pub fn run_tray(engine_hwnd : usize) -> Result<Child, std::io::Error>{
    let result = Command::new("ranami-wallpapers-tray.exe")
                        .arg(engine_hwnd.to_string())
                        ;
    todo!()
}