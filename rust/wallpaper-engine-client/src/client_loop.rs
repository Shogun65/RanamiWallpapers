pub mod init_loop{

    use crate::namepipe::init_namepipe;
    use tokio::runtime::Runtime;
    use core::time;
    use std::{sync::{Arc, Mutex}, thread};
    use shared::{log_err::err_log, namepipe::NamePipeCommands};
    use crate::init_gui::init_gui;
    use crate::engine::init_engine::run_wallpaper_engine;
    use std::process::Child;

    pub fn run(
        client_hwnd : usize,
        namepipecommands : Arc<Mutex<NamePipeCommands>>
    ) -> Result<(), std::io::Error>
    {
        let runtime = Runtime::new()?;
        
        let runtime_handle = runtime.handle(); 

        let handle_pipe = init_namepipe::run_namepipe_server(
                                        namepipecommands.clone(), runtime_handle);
        println!("ID of run_namepipe_server: {}", handle_pipe.id());

        let child_of_gui = init_gui::run_gui()?;
        println!("PID OF GUI: {}", child_of_gui.id());

        run_2(client_hwnd, namepipecommands.clone())?;

        // loop {
        //     thread::sleep(time::Duration::from_secs(1));
        //     println!("{:#?}", namepipecommands);
            
        // } 

        return Ok(());
    }

    fn run_2(
        client_hwnd : usize,
        namepipecommands : Arc<Mutex<NamePipeCommands>>
    ) -> Result<(), std::io::Error>
    {
        let mut ranami_child: Option<Child> = None;

        loop {
            {
                let mut state = namepipecommands.lock();

                if let Err(err) = state{
                    err_log(&format!("err on state of run_2 : {}", err));
                    sleep_300_micros();
                    continue;
                }

                let mut state = state.unwrap();

                if state.wallpaper_changed == true{

                   let ranamichild = run_wallpaper_engine(&state.video_path, "3", client_hwnd)?;
                    
                    if ranami_child.is_none(){
                        ranami_child = Some(ranamichild);
                    }
                    
                    state.wallpaper_changed = false; // do not forget this and i mean it dont forget this
                }
            }
            sleep_300_micros();
        }
        

        return Ok(());
    }

    fn run_3(){}

    fn sleep_300_micros(){
        thread::sleep(time::Duration::from_micros(300));
    }
    
}