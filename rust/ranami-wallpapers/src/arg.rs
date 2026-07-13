pub(crate) mod init {
    use windows::Win32::System::Console::AllocConsole;

    use super::error::ErrorArg;

    pub(crate) fn init() -> Result<(), ErrorArg> {
        init_console()?;
        return Ok(());
    }

    fn init_console() -> Result<(), ErrorArg> {
        if std::env::args().any(|arg| arg == "--console") {
            unsafe {
                let console_result = AllocConsole();

                match console_result {
                    Ok(_) => {
                        println!("[INFO] Init console Done!");
                        return Ok(());
                    }

                    Err(err) => return Err(ErrorArg::ConsoleErr(err.to_string())),
                }
            };
        }
        return Ok(());
    }
}

pub(crate) mod error {

    pub(crate) enum ErrorArg {
        ConsoleErr(String),
    }
}
