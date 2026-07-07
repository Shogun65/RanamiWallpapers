pub mod init_loop{

    use crate::namepipe::init_namepipe;
    use tokio::runtime::Runtime;
    use core::time;
    use std::{sync::{Arc, Mutex}, thread};
    use shared::namepipe::NamePipeCommands;

    pub fn run(
        client_hwnd : usize,
        namepipecommands : Arc<Mutex<NamePipeCommands>>
    ) -> Result<(), std::io::Error>
    {
        println!();
        let runtime = Runtime::new()?;
        
        let runtime_handle = runtime.handle(); 

        let _handle_pipe = init_namepipe::run_namepipe_server(
                                        namepipecommands.clone(), runtime_handle);

        loop {
            thread::sleep(time::Duration::from_secs(1));
            println!("{:#?}", namepipecommands);
        } 

        return Ok(());
    }

    fn run_1(){
        
    }
    
}