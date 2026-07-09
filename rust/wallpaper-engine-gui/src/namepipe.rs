pub mod namepipe{

    use tokio::io::AsyncWriteExt;
    use tokio::net::windows::named_pipe::ClientOptions;
    use shared::namepipe::{NamePipeCommands, PIPE_NAME};
    use shared::log_err::err_log;
    use serde_json;
    use tokio::runtime::{Handle, Runtime};

    pub fn sent_struct_of_data_to_client(video_path: String, handle : &Handle)
    {
        handle.spawn(async move {

            let result = sent_data_to_client(video_path).await;

            match result {

                Ok(_) => return,

                Err(err) => {
                    err_log(&format!("error on sent_data_to_client: {:#?}", err.raw_os_error()));
                    return;
                }
            }
        });
    }

    pub fn get_runtime() -> Option<Runtime>{

        let runtime = Runtime::new();

        if let Err(err) = runtime{
            err_log(&format!("error on Runtime::new(): {}", err));
            return None;
        }; // idk what iam doing i feel stupid

        return Some(runtime.unwrap());
    }

    async fn sent_data_to_client(video_path: String) -> std::io::Result<()>
    {

        println!("PIPE_NAME(client): {}", PIPE_NAME);
        let mut namepipe = ClientOptions::new()
                                        .open(PIPE_NAME)?;

        let video_path = get_json_of_struct(video_path)?;

        namepipe.write_all(video_path.as_bytes()).await?;

        return Ok(());
    }

    fn get_json_of_struct(video_path: String) -> Result<String, std::io::Error>{

        let namepipe = NamePipeCommands{
            video_path : video_path, 
            wallpaper_changed : true
        };

        println!("[DEBUG] namepipe: {:#?}", namepipe);

        let json = serde_json::to_string(&namepipe)?;

        println!("[DEBUG] namepipe json: {}", json);

        return Ok(json);
    }

}