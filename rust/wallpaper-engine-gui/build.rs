use shared::log_err::err_log;

fn main(){

    let compiler_result = slint_build::compile("ui/main-window.slint");
    
    match compiler_result {
        Ok(_) => {},
        Err(err) => {err_log(&format!("Error on slint compiler: {}", err)); return;},
    }
}