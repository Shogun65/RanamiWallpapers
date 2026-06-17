pub(crate) mod client_init
{
    use std::path::Path;

    
    pub fn client_init() -> Result<(), super::error::Error>
    {
        todo!("still in work");
    }


    fn check_files() -> Result<(), super::error::Error>
    {
        use super::log_err::log;
        use super::files::FILES_CHECK_LIST;

        for file in FILES_CHECK_LIST{
            if !Path::new(file).exists()
            {
                log(&format!("MISSING FILE: {}", file))?;
                return Err(super::error::Error::MissingFile);
            }
            println!("File found: {}", file);
        }
        return Ok(());
    }



}

mod files
{
    pub(super) const FILES_CHECK_LIST: [&str; 9] = ["swscale-8.dll", "swresample-5.dll", 
                                                    "postproc-58.dll", "avutil-59.dll",
                                                    "avformat-61.dll", "avfilter-10.dll",
                                                    "avdevice-61.dll", "avcodec-61.dll",
                                                    "AliveWallpaperEngine.exe"];
}

pub(crate) mod log_err
{
    use std::fs::OpenOptions;
    use std::io::Write;


    pub fn log(message: &str) -> Result<(), super::error::Error>
    {
        let debug_file = OpenOptions::new()
                                                .create(true)
                                                .append(true)
                                                .open("debug.txt");

        let result = match debug_file {

            Ok(mut debug) => writeln!(debug, "[ERROR] {}", message),
            Err(_) => return Err(super::error::Error::CantOpenDebugFile),

        };

        match result {

            Ok(_) => return Ok(()),
            Err(_) => return Err(super::error::Error::CantWriteDebugError),

        };

    }
}

pub mod error
{
    pub enum Error {
        MissingFile,
        CantWriteDebugError,
        CantOpenDebugFile,
    }
}