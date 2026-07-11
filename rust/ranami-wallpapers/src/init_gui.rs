pub mod init_gui{

    use std::process::{Child, Command};

    pub fn run_gui() -> Result<Child, std::io::Error>
    {
        let gui_exe = Command::new("ranami-wallpapers-gui.exe").spawn()?;
        return Ok(gui_exe);
    }
}