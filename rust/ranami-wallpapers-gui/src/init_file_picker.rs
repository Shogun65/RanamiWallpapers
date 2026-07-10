pub(crate) mod init {
    use rfd::{AsyncFileDialog, FileDialog, FileHandle};
    use std::path::PathBuf;

    #[allow(dead_code)]
    pub(crate) fn init_s_file_picker() -> Option<PathBuf> {
        // Older sync picker kept around for quick tests outside async Slint callbacks.
        let file = FileDialog::new()
            .add_filter("video", &["mp4", "mkv", "avi", "mov", "wmv"])
            .set_directory("/")
            .set_title("Alive-Wallpaper-Engine")
            .pick_file();

        return file;
    }

    pub async fn init_a_file_picker() -> Option<FileHandle> {
        // This is the picker the GUI uses so imports can happen without blocking the window thread.
        let file_handle = AsyncFileDialog::new()
            .add_filter("video", &["mp4", "mkv", "avi", "mov", "wmv"])
            .set_directory("/")
            .set_title("Alive-Wallpaper-Engine")
            .pick_file()
            .await;

        return file_handle;
    }
}
