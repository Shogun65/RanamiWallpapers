use shared::log_err::err_log;
use slint_build::CompileError;

fn main() {
    let compiler_result = slint_build::compile("ui/main-window.slint");
    check_compiler_result(compiler_result);
}

fn check_compiler_result(compiler_result: Result<(), CompileError>) {
    match compiler_result {
        Ok(_) => {}
        Err(err) => {
            err_log(&format!("Error on slint compiler: {}", err));
            panic!("Slint compile failed: {}", err);
        }
    }
}
