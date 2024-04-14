use std::fs::{self, File, Permissions};
use std::io::Write;
use std::path::PathBuf;

use color_eyre::eyre::{bail, Report};
use color_eyre::{eyre::Context, Result};
use dirs::cache_dir;

use crate::utility::task;
use crate::utility::http;

pub(crate) struct DownloadBootstrapLinux;

impl task::Task for DownloadBootstrapLinux {
    fn name(&self) -> String {
        "download_bootstrap_linux".to_owned()
    }

    fn execute(&self) -> Result<()> {
        let bytes = http::download("https://github.com/johannes-qvarford/home-env-bootstrap/releases/latest/download/bootstrap").context("Downloading bootstrap-linux binary")?;
        //let mut path = cache_directory().context("Cache directory for download bootstrap-linux binary to.")?;
        //path.push("bootstrap");
        let path = PathBuf::from(r#"\\wsl.localhost\Ubuntu\home\jq\bootstrap"#);
        println!("Path is {path:?}");
        let mut file = File::create(path).context("Creating bootstrap linux file")?;
        file.write_all(&bytes).context("Writing to bootstrap linux file")?;
        
        // Note: Windows can't mark it as executable, we need to shell out to wsl.exe to do that.
        //make it executable.

        //share the filename between tasks.
        //maybe have both tasks in the same file?
        //then execute file through wsl.exe -e /path/to/bin/x
        Ok(())
    }

    fn requires_restart(&self) -> bool {
        false
    }
}

// TODO: Centralize directory handling?
fn cache_directory() -> Result<PathBuf> {
    let mut dir =
        cache_dir().ok_or(Report::msg("Cache directory was not found"))?;
    dir.push("home-env-bootstrap");
    Ok(dir)
}

pub(crate) fn download_bootstrap_linux_task() -> Box<dyn task::Task> {
    Box::new(DownloadBootstrapLinux)
}