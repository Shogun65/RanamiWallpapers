pub mod init_namepipe{

    use shared::namepipe::{NamePipeCommands, PIPE_NAME};
    use tokio::{io::AsyncReadExt, runtime::Handle};
    use tokio::net::windows::named_pipe::ServerOptions;
    use std::sync::{Mutex, Arc};
    use tokio::task::JoinHandle;
    use serde_json;

    pub fn run_namepipe_server(

        namepipecommands : Arc<Mutex<NamePipeCommands>>,
        handle : &Handle

    ) -> JoinHandle<()> // idk maybe we use in future
    {
        return handle.spawn(async move{
            loop{
                println!("PIPE_NAME(server): {}", PIPE_NAME);
                let mut pipe = ServerOptions::new().create(PIPE_NAME).unwrap(); // it not going to failed cmom man

                println!("[DEBUG] pipe is waiting to Connection!");

                let result = pipe.connect().await;

                println!("result pipe connectL {:?}", result);

                println!("[DEBUG] pipe in conneted!");

                let mut buffer = [0u8; 4096];

                let bytes = pipe.read(&mut buffer).await.unwrap();

                let json = String::from_utf8_lossy(&buffer[..bytes]).to_string();

                let namepipe: NamePipeCommands = serde_json::from_str(&json).unwrap();

                let mut state = namepipecommands.lock().unwrap();

                state.video_path = namepipe.video_path;
                state.wallpaper_changed = namepipe.wallpaper_changed;
            }
        });
    }
}