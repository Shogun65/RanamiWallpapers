pub(crate) mod init{
    use windows::Win32::System::Console::AllocConsole;

    use super::error::Error;

    pub(crate) fn init() -> Result<(), Error>
    {
        init_console()?;
        return Ok(());
    }

    fn init_console() -> Result<(), Error>{
        if std::env::args().any(|arg| arg == "--console") {
            unsafe {
                let console_result = AllocConsole();

                match console_result {
                    Ok(_) => {
                        println!("[INFO] Init console Done!");
                        return Ok(());
                    },

                    Err(err) => return Err(Error::ConsoleErr(err.to_string())), 
                }
            };
        }
        return Ok(());
    }
}

pub(crate) mod error{

    pub(crate) enum Error {
        ConsoleErr(String),
    }

}