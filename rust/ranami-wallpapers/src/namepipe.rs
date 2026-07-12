pub mod init_namepipe{

    use shared::namepipe::{NamePipeCommands, PIPE_NAME};
    use tokio::{io::AsyncReadExt, runtime::Handle};
    use tokio::net::windows::named_pipe::ServerOptions;
    use std::sync::{Mutex, Arc};
    use tokio::task::JoinHandle;
    use serde_json;
    use shared::log_err::err_log;

    pub fn run_namepipe_server(

        namepipecommands : Arc<Mutex<NamePipeCommands>>,
        handle : &Handle

    ) -> JoinHandle<()> // idk maybe we use in future
    {
        return handle.spawn(async move{
            loop{
                println!("PIPE_NAME(server): {}", PIPE_NAME);
                let mut pipe = match ServerOptions::new().create(PIPE_NAME){

                    Ok(pipe) => pipe,

                    Err(err) =>{
                        err_log(&format!("Error on creating the pipe: {}", err));
                        continue;
                    }
                };

                println!("[DEBUG] pipe is waiting to Connection!");

                let result = pipe.connect().await;

                if let Err(err) =  result {
                    err_log(&format!("Err on pipe.connect: {}", err));
                    continue;
                }

                println!("[DEBUG] pipe in conneted!");

                let mut buffer = [0u8; 8192];

                let bytes = match pipe.read(&mut buffer).await{
                    Ok(byte) => byte,
                    Err(err) => {
                        err_log(&format!("Error on pipe.read: {}", err));
                        continue;
                    }
                };

                let json = String::from_utf8_lossy(&buffer[..bytes]);

                let namepipe: NamePipeCommands = match serde_json::from_str(&json){
                    Ok(data) => data,
                    Err(err) => {
                        err_log(&format!("Error on serde_json::from_str(&json): {}", err));
                        continue;
                    }
                };

                let mut state = match namepipecommands.lock(){
                    Ok(lock) => lock,
                    Err(err) =>{
                        err_log(&format!("Mutex Err on namepipe(server): {}", err));
                        err.into_inner()
                    }
                };

                *state = namepipe; // they both are same struct soo yeaa
                println!("Update state(server): {:?}", state);                

            }
        });
    }
}