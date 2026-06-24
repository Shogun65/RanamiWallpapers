pub(crate) mod client_init
{
    use std::path::Path;
    
    pub fn client_init() -> Result<(), super::error::ErrorClient>
    {
        check_files()?;

        return Ok(());

    }


    fn check_files() -> Result<(), super::error::ErrorClient>
    {
        use super::files::FILES_CHECK_LIST;
        // use super::log_err::err_log;

        for file in FILES_CHECK_LIST{
            if !Path::new(file).exists()
            {
                //err_log(&format!("MISSING FILE: {}", file));
                return Err(super::error::ErrorClient::MissingFile(String::from(file)));
            }
            println!("[INFO] File found: {}", file);
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


    pub fn err_log(message: &str) // useing panic! here wont hurt because 
    //                              it dont matter when we do err_log we already want our app to get exit
    {
        let debug_file = OpenOptions::new()
                                                .create(true)
                                                .append(true)
                                                .open("debug.txt");

        let result = match debug_file {

            Ok(mut debug) => writeln!(debug, "[ERROR] {}", message),
            Err(_) => panic!("CantOpenDebugFile"),

        };

        match result {

            Ok(_) => {},
            Err(_) => panic!("CantWriteDebugError"),

        };

    }
}

pub mod error
{
    // pub(super) enum ErrorLog 
    // {
    //     CantWriteDebugError,
    //     CantOpenDebugFile,
    // }

    pub enum ErrorClient
    {
        MissingFile(String),
    }
}