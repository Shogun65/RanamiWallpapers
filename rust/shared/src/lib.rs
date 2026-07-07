// check "docs/" for more info!

// Custom engine messages.
// WM_APP = 0x8000
//

pub mod message {
    pub(crate) const WM_APP: u32 = 0x8000; // only for shared crate

    // and if you want anythink new always make a new value and doc it (if you remember hehe)
    pub const WM_ENGINE_TEST: u32 = WM_APP + 1; // 0x8000 + 1 = 0x8001
    pub const WM_ENGINE_EXIT: u32 = WM_APP + 2;
    pub const WM_ENGINE_BOOTUP_SUCCESS: u32 = WM_APP + 3;
    pub const WM_ENGINE_BOOTUP_FAILED: u32 = WM_APP + 4;
}

pub mod log_err {
    use std::fs::OpenOptions;
    use std::io::Write;

    pub fn err_log(message: &str)
    // useing panic! here wont hurt because
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
            Ok(_) => {}
            Err(_) => panic!("CantWriteDebugError"),
        };
    }
}

pub mod save_wallpaper {

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Default, Serialize, Deserialize)]
    pub struct SaveWallpaper {
        pub name: String,
        pub path: String,
    }
}

pub mod namepipe{

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct NamePipeCommands{
        pub video_path: String,
    }

    pub const PIPE_NAME: &str = r"\\.\pipe\RanamiWallpapers";
}
