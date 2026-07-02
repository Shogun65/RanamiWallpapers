pub(crate) mod init{

    use std::path::PathBuf;
    use rfd::{AsyncFileDialog, FileDialog, FileHandle};

    pub(crate) fn init_s_file_picker() -> Option<PathBuf> {


        let file = FileDialog::new()
                                .add_filter("video", &["mp4", "mkv", "avi", "mov","wmv"])
                                .set_directory("/")
                                .set_title("Alive-Wallpaper-Engine")
                                .pick_file();
    
        return file;
    
    }

    #[allow(dead_code)]
    pub async fn init_a_file_picker() -> Option<FileHandle>{

        let file_handle = AsyncFileDialog::new()
                                .add_filter("video", &["mp4", "mkv", "avi", "mov","wmv"])
                                .set_directory("/")
                                .set_title("Alive-Wallpaper-Engine")
                                .pick_file().await;
        
        return file_handle;
    }


}