use std::fs::{self, File, Permissions};
use std::io::Write;
use std::path::PathBuf;

use color_eyre::eyre::{bail, Report};
use color_eyre::{eyre::Context, Result};
use dirs::cache_dir;

use crate::utility::{process, task};
use crate::utility::http;

pub(crate) struct DownloadBootstrapLinux;

impl task::Task for DownloadBootstrapLinux {
    fn name(&self) -> String {
        "download_bootstrap_linux".to_owned()
    }

    fn execute(&self) -> Result<()> {
        let bytes = http::download("https://github.com/johannes-qvarford/home-env-bootstrap/releases/latest/download/bootstrap").context("Downloading bootstrap-linux binary")?;
        let path = PathBuf::from(bootstrap_binary_path);
        println!("Path is {path:?}");
        let mut file = File::create(path).context("Creating bootstrap linux file")?;
        file.write_all(&bytes).context("Writing to bootstrap linux file")?;
        process::execute("wsl.exe", &["-e", "bash", "-c", &format!("chmod 755 {bootstrap_binary_path_linux}")], &[]).wrap_err("Adding the executable permission to bootstrap-linux binary")?;
        Ok(())
    }

    fn requires_restart(&self) -> bool {
        false
    }
}


pub(crate) fn download_bootstrap_linux_task() -> Box<dyn task::Task> {
    Box::new(DownloadBootstrapLinux)
}

pub(crate) struct RunBootstrapLinux;

impl task::Task for RunBootstrapLinux {
    fn name(&self) -> String {
        "run_bootstrap_linux".to_owned()
    }

    fn execute(&self) -> Result<()> {
        process::execute("wsl.exe", &["-e", "bash", "-c", &format!("{} --task bash_temp", bootstrap_binary_path_linux)], &[]).wrap_err("Running bootstrap-linux binary")?;
        Ok(())
    }

    fn requires_restart(&self) -> bool {
        false
    }
}


pub(crate) fn run_bootstrap_linux_task() -> Box<dyn task::Task> {
    Box::new(RunBootstrapLinux)
}

// TODO: Centralize directory handling?
fn cache_directory() -> Result<PathBuf> {
    let mut dir =
        cache_dir().ok_or(Report::msg("Cache directory was not found"))?;
    dir.push("home-env-bootstrap");
    Ok(dir)
}

const bootstrap_binary_path: &str = r#"\\wsl.localhost\Ubuntu\home\jq\bootstrap"#;
const bootstrap_binary_path_linux: &str = "/home/jq/bootstrap";