mod init_s_file_picker;
mod file_saver;

use init_s_file_picker::init::init_s_file_picker;
use file_saver::file_saver::save_file_1;

use shared::log_err::err_log;

fn main() {
    let video_path = init_s_file_picker();

    println!("video_path: {:?}", video_path);

    match video_path {
        
        Some(video_path_found) => {
            let save_result = save_file_1(video_path_found);

            match save_result {
                Ok(_) => {},
                Err(err) => { err_log(&format!("{}", err)); return;}
            }
        },

        None => {}
    }

}
