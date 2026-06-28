pub(crate) mod init{

    use std::path::PathBuf;
    use rfd::FileDialog;

    pub(crate) fn init_s_file_picker() -> Option<PathBuf> {


        let file = FileDialog::new()
                                .add_filter("video", &["mp4", "mkv", "avi", "mov","wmv"])
                                .set_directory("/")
                                .set_title("Alive-Wallpaper-Engine")
                                .pick_file();
    
        match file {
            Some(path_to_video) =>{
                return Some(path_to_video);
            },

            None => {
                return None;
            },
        }
    
    }


}