/*Invoke-WebRequest https://i.redd.it/r235334p4tl61.png -OutFile "~/Pictures/terminal_background.png"

New-Item -ItemType SymbolicLink `
-Path "$HOME\AppData\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json" `
-Target "..\terminal\settings.json" */

use std::{fs::File, io::Write, path::Path};

use color_eyre::{
    eyre::{Context, Report},
    Result,
};

use crate::{
    utility::symlink,
    utility::task::{self},
};

pub(crate) struct ConnectWindowsTerminal;

impl task::Task for ConnectWindowsTerminal {
    fn name(&self) -> String {
        "connect_windows_terminal".to_owned()
    }

    fn execute(&self) -> Result<()> {
        let mut terminal_path = dirs::data_local_dir().unwrap();
        terminal_path.push("Packages");
        terminal_path.push("Microsoft.WindowsTerminal_8wekyb3d8bbwe");
        terminal_path.push("LocalState");
        terminal_path.push("settings.json");

        let home_env_path = Path::new(r#"\\wsl$\Ubuntu\home\jq\home-env\terminal\settings.json"#);

        // TODO: SymbolicLink
        symlink::symlink_file(
            terminal_path.to_str().unwrap(),
            home_env_path.to_str().unwrap(),
        )
        .wrap_err_with(|| format!("Symlinking {home_env_path:?} to {terminal_path:?}"))?;

        let pic_url = "https://i.redd.it/r235334p4tl61.png";
        let pic_response = reqwest::blocking::get(pic_url)
            .wrap_err_with(|| format!("Fetching terminal pic from: {pic_url}"))?;
        let status = pic_response.status();

        if !status.is_success() {
            return Err(Report::msg(format!(
                "Unsuccessful response when fetching terminal pic url: {status}"
            )));
        }

        let bytes = pic_response
            .bytes()
            .wrap_err("Converting response to bytes")?;

        let pic_dir = dirs::picture_dir().ok_or(Report::msg("Could not determine picture directory.".to_string()))?;
        let mut pic_file = pic_dir.clone();
        pic_file.push("terminal_background.png");
        let mut file = File::create(pic_file)
            .wrap_err_with(|| format!("Creating picture at '{pic_dir:?}'"))?;
        file.write_all(&bytes)
            .wrap_err("Writing bytes to picture")?;

        Ok(())
    }
}

pub(crate) fn connect_windows_terminal() -> Box<dyn task::Task> {
    Box::new(ConnectWindowsTerminal)
}
