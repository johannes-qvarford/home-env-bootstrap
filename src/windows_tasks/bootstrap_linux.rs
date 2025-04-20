use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use color_eyre::{eyre::Context, Result};

use crate::utility::http;
use crate::utility::{process, task};

pub(crate) struct DownloadBootstrapLinux;

impl task::Task for DownloadBootstrapLinux {
    fn name(&self) -> String {
        "download_bootstrap_linux".to_owned()
    }

    fn execute(&self) -> Result<()> {
        let bytes = http::download("https://github.com/johannes-qvarford/home-env-bootstrap/releases/latest/download/bootstrap").context("Downloading bootstrap-linux binary")?;
        let path = PathBuf::from(BOOTSTRAP_BINARY_PATH);
        println!("Path is {path:?}");
        let mut file = File::create(path).context("Creating bootstrap linux file")?;
        file.write_all(&bytes)
            .context("Writing to bootstrap linux file")?;
        process::execute(
            "wsl.exe",
            &[
                "-e",
                "bash",
                "-c",
                &format!("chmod 755 {BOOTSTRAP_BINARY_PATH_LINUX}"),
            ],
            &[],
        )
        .wrap_err("Adding the executable permission to bootstrap-linux binary")?;
        Ok(())
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
        process::execute(
            "wsl.exe",
            &[
                "-e",
                "bash",
                "-c",
                &format!("{}", BOOTSTRAP_BINARY_PATH_LINUX),
            ],
            &[],
        )
        .wrap_err("Running bootstrap-linux binary")?;
        Ok(())
    }
}

pub(crate) fn run_bootstrap_linux_task() -> Box<dyn task::Task> {
    Box::new(RunBootstrapLinux)
}

const BOOTSTRAP_BINARY_PATH: &str = r#"\\wsl.localhost\Ubuntu\home\jq\bootstrap"#;
const BOOTSTRAP_BINARY_PATH_LINUX: &str = "/home/jq/bootstrap";
