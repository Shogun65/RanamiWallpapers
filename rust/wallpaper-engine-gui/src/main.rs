slint::include_modules!();

mod init_file_picker;
mod file_saver;

// use init_file_picker::init::init_s_file_picker;
// use file_saver::file_saver::{save_file_1, read_file_1};
// use shared::log_err::err_log;



fn main() -> Result<(), slint::PlatformError> {

    let ui = AppWindow::new()?;


    return ui.run();

}
