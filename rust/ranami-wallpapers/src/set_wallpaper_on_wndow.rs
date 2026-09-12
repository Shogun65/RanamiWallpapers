pub mod set_wallpaper{

	use std::process::{Command, Stdio};
	use std::path::{PathBuf, Path};
	use shared::save_path_and_settings::{
		CREATE_NO_WINDOW,
		THUMBNAIL_TIMESTAMP,
		static_wallpapers_cache_dir_path,
		STATIC_WALLPAPER_EXTENSION
	};
	use std::os::windows::process::CommandExt;
	use std::ffi::OsStr;
    use std::thread::{self, JoinHandle};
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW,
        SPI_SETDESKWALLPAPER,
        SPIF_SENDCHANGE,
        SPIF_UPDATEINIFILE};
    use windows::core::HSTRING;

    use shared::log_err::err_log;

     fn generate_static_wallpaper(video_path: &Path) -> Result<(), String> {

     	let (w, h) : (u64, u64) = get_primary_screen_size()?;
     	let static_wallpaper_path = get_static_wallpaper_save_path(&video_path)?;

        let ffmpeg_command = PathBuf::from("ffmpeg.exe");
        // ffmpeg.exe is a console application, so spawn it with CREATE_NO_WINDOW to stop
        // Windows from flashing a temporary CMD window while thumbnails are being built.
        let status = hidden_tool_command(&ffmpeg_command)
            .arg("-y")
            .arg("-ss")
            .arg(THUMBNAIL_TIMESTAMP) // we use the same TIMESTAMP for wallpaper to
            .arg("-i") 	// so user can see the perview as THUMBNAIL that show on GUI 
            .arg(video_path)
            .arg("-frames:v")
            .arg("1")
            .arg("-vf")
            .arg(format!(
                "scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h}"
            ))
            .arg("-q:v")
            .arg("3")
            .arg(&static_wallpaper_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|err| {
                format!(
                    "Could not start ffmpeg command '{}' for {}: {err}",
                    ffmpeg_command.display(),
                    video_path.display()
                )
            })?;

        if !status.success() {
            return Err(format!(
                "ffmpeg failed while generating thumbnail for {}, status code: {}",
                video_path.display(), status.code().unwrap_or(2627)
            ));
        }

        if !static_wallpaper_path.exists() {
            return Err(format!(
                "ffmpeg reported success but no thumbnail was created at {}",
                static_wallpaper_path.display()
            ));
        }

        return Ok(());
    }

    fn hidden_tool_command(program: impl AsRef<OsStr>) -> Command {
        let mut command = Command::new(program);
        command.creation_flags(CREATE_NO_WINDOW);
        return command;
    }

    // first is "W" and second is "H"
    fn get_primary_screen_size() -> Result<(u64, u64), String>{
    	return screen_size::get_primary_screen_size()
    				.map_err(|err|{
    					format!("Err on get_primary_screen_size: {err}")
    				});
    }

    fn get_static_wallpaper_save_path(video_path: &Path) -> Result<PathBuf, String> {
    	let static_wallpaper_dir = static_wallpapers_cache_dir_path().map_err(|err|
    		{
    			return format!("Err on get_static_wallpaper_save_path: {err}");
    		})?;

    	let static_wallpaper_name = static_wallpaper_dir.join(format!("{}_{}.{}",
    		stable_path_hash(&video_path.to_string_lossy()),
    		&video_path.file_stem().and_then(|str| str.to_str()).unwrap_or("err"),
    		STATIC_WALLPAPER_EXTENSION));

    	return Ok(static_wallpaper_name);
    }

    fn stable_path_hash(path: &str) -> String {// idk how it work talk to codex
        // A tiny stable hash is enough here; it just prevents thumbnail file name collisions.
        let mut hash: u64 = 0xcbf29ce484222325;

        for byte in path.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        format!("{hash:016x}")
    }

    pub fn set_static_wallpaper(video_path: PathBuf){

            if need_genration(&video_path){
                println!("[INFO] need_genration");
                let result = generate_static_wallpaper(&video_path);

                match result {
                    Ok(_) => {},
                    Err(err) => {println!("Err on generate_static_wallpaper: {err}"); return;},
                }
            }
            else{
                println!("[INFO] DONT need_genration");
            }

            let static_wallpaper_path = get_static_wallpaper_save_path(&video_path);

            let static_wallpaper_path = match static_wallpaper_path {
                Ok(path) => path,
                Err(err) => {println!("Err on generate_static_wallpaper: {err}"); return;},
            };

            if !set_static_wallpaper_on_desktop(&static_wallpaper_path){
                println!("Err Cant set the static wallpaper! Check error.txt");
            }
    }

    fn need_genration(video_path: &Path) -> bool {

        let wallpaper_full_path_hash = get_static_wallpaper_save_path(&video_path);
    
        let wallpaper_full_path_hash = match wallpaper_full_path_hash {
            Ok(path) => path,
            Err(err) =>{
                println!("Err on need_genration: {err}");
                let a = PathBuf::new();
                a
            },
        };

        if wallpaper_full_path_hash.exists(){ return false;}// it dont need

        return true;// needed to genrated
    }

    fn set_static_wallpaper_on_desktop(video_path: &Path) -> bool{

        let utf_16_path = HSTRING::from(video_path);
        unsafe {
            let result = SystemParametersInfoW(
                    SPI_SETDESKWALLPAPER,
                    0, 
                    Some(utf_16_path.as_ptr() as *mut std::ffi::c_void), 
                    SPIF_UPDATEINIFILE | SPIF_SENDCHANGE);

            match result {
                Ok(_) => return true,
                Err(err) =>{
                    err_log(&format!("Err on SystemParametersInfoW: {err}"));
                    return false;
                }
            }
        }
    }

}