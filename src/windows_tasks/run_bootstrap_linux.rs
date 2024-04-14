use color_eyre::{eyre::Context, Result};

use crate::{utility::process, utility::task};

pub(crate) struct RunBootstrapLinux;

impl task::Task for RunBootstrapLinux {
    fn name(&self) -> String {
        "run_bootstrap_linux".to_owned()
    }

    fn execute(&self) -> Result<()> {
        //process::execute("wsl.exe", &["--install"], &[]).wrap_err("Installing WSL".to_string())
        Ok(())
    }

    fn requires_restart(&self) -> bool {
        false
    }
}

pub(crate) fn run_bootstrap_linux_task() -> Box<dyn task::Task> {
    Box::new(RunBootstrapLinux)
}