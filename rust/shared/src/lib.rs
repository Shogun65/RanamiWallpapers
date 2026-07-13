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
    pub const WM_ENGINE_SENT_HWND: u32 = WM_APP + 5;
    pub const WM_ENGINE_OPEN_GUI: u32 = WM_APP + 6;
    pub const WM_ENGINE_D3D11_FMT_NOT_FOUND: u32 = WM_APP + 7;
}

pub mod log_err {
    use std::fs::OpenOptions;
    use std::io::Write;

    pub fn err_log(message: &str) {
        let debug_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.txt");

        let result = match debug_file {
            Ok(mut debug) => writeln!(debug, "[ERROR] {}", message),
            Err(_) => return,
        };

        match result {
            Ok(_) => {}
            Err(_) => {}
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

pub mod namepipe {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct NamePipeCommands {
        pub video_path: String,
        pub wallpaper_changed: bool,
    }

    pub const PIPE_NAME: &str = r"\\.\pipe\RanamiWallpapers";
}

pub mod save_path_and_settings {
    pub const SAVE_WALLPAPERS_PATH: &str = "Save-Wallpapers.json";
    pub const THUMBNAIL_CACHE_DIR: &str = "cache-wallpaper-thumbnails";
    pub const THUMBNAIL_EXTENSION: &str = "jpg";
    pub const THUMBNAIL_WIDTH: &str = "640";
    pub const THUMBNAIL_HEIGHT: &str = "360";
    pub const THUMBNAIL_TIMESTAMP: &str = "00:00:01";
    pub const STARTUP_FILE_SAVE_NAME: &str = "RanamiWallpapers-startup-file.txt";
    pub const CREATE_NO_WINDOW: u32 = 0x08000000;
}

pub mod error_code_core {
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EngineFailureCode {
        None = 0,

        StartupUnknown = 10,
        StartupParseArgsFailed = 11,
        StartupOpenInputFailed = 12,
        StartupStreamInfoFailed = 13,
        StartupD3D11FormatMissing = 14,
        StartupHwDeviceInitFailed = 15,
        StartupHwFramesInitFailed = 16,
        StartupCodecOpenFailed = 17,
        StartupDxvaInitFailed = 18,
        StartupSwapChainFailed = 19,
        VideoPathInvalid = 20,
        RuntimeUnknown = 30,

        RuntimeDecoderLoopFailed = 31,
        RuntimeVideoProcessorFailed = 32,
        RuntimePresentFailed = 33,
        RuntimeDeviceLost = 34,
    }
}
