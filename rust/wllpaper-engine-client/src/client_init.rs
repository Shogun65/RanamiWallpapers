pub(crate) mod client_init
{
    use std::path::Path;

    
    pub fn client_init() -> Result<(), String>
    {
        todo!("still in work");
    }


    fn check_files()
    {
        use super::log_err::log;
        use super::files::FILES_CHECK_LIST;

        for file in FILES_CHECK_LIST{
            if !Path::new(file).exists()
            {
                log(&format!("MISSING FILE: {}", file));
            }
            println!("File found: {}", file);
        }
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


    pub fn log(message: &str)
    {
        let debug_file = OpenOptions::new()
                                                .create(true)
                                                .append(true)
                                                .open("debug.txt");

        let result = match debug_file {

            Ok(mut debug) => writeln!(debug, "[ERROR] {}", message),
            Err(err) => panic!("Error happand in engine also cant right log!, ERROR {}", err),

        };

        match result {

            Ok(_) => {},
            Err(err) => panic!("Cant write the ERROR: {}", err),

        };

    }
}