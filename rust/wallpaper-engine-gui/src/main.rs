slint::include_modules!();

mod init_file_picker;
mod file_saver;

use init_file_picker::init::init_a_file_picker;
// use file_saver::file_saver::{save_file_1, read_file_1};
// use shared::log_err::err_log;



fn main() -> Result<(), slint::PlatformError> {

    let ui = AppWindow::new()?;

    ui.on_import_wallpaper(||{
        
        slint::spawn_local(async move{

            let file_handle = init_a_file_picker().await;

            if let Some(file_handle) = file_handle{
                // save the path as json and wallpaper name too
            }

        }).unwrap();
    });

    return ui.run();

}
