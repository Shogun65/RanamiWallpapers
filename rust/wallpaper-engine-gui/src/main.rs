slint::include_modules!();

mod init_file_picker;
mod file_saver;

use init_file_picker::init::init_a_file_picker;
use shared::log_err::err_log;
use crate::file_saver::file_saver::save_file_2;

fn main() -> Result<(), slint::PlatformError> {

    let ui = AppWindow::new()?;

    ui.on_import_wallpaper(||{
        
        slint::spawn_local(async move{

            let file_handle = init_a_file_picker().await;

            if let Some(file_handle) = file_handle{
                if let Err(err) = save_file_2(file_handle){
                    err_log(&format!("Error on save_file_2: {}", err));
                }
            }

        }).unwrap();
    });

    return ui.run();

}
