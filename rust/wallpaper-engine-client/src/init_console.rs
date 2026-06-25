pub mod init_console{
    use windows::Win32::System::Console::AllocConsole;

    pub fn init_console(){
        if std::env::args().any(|arg| arg == "--console") {
            unsafe {
                AllocConsole().ok();
            };
        }
    }
}