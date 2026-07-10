use std::process::{Child, Command};


pub fn run_tray(client_hwnd : usize) -> Result<Child, std::io::Error>{
    return Command::new("ranami-wallpapers-tray.exe")
                        .arg(client_hwnd.to_string()).spawn();
    
}