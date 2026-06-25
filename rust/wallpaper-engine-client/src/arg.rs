pub(crate) mod init{
    use windows::Win32::System::Console::AllocConsole;

    use super::error::ConsoleError;

    pub(crate) fn init() -> Result<(), super::error::ConsoleError>
    {
        init_console()?;
        return Ok(());
    }

    fn init_console() -> Result<(), super::error::ConsoleError>{
        if std::env::args().any(|arg| arg == "--console") {
            unsafe {
                let console_result = AllocConsole();

                match console_result {
                    Ok(_) => {
                        println!("[INFO] Init console Done!");
                        return Ok(());
                    },

                    Err(err) => return Err(ConsoleError::ConsoleErr(err.to_string())), 
                }
            };
        }
        return Ok(());
    }
}

pub(crate) mod error{

    pub(crate) enum ConsoleError {
        ConsoleErr(String),
    }

}