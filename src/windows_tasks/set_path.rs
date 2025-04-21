use color_eyre::{eyre::Context, Result};

use crate::{
    utility::process,
    utility::task::{self},
};

pub(crate) struct SetPathTask;

impl task::Task for SetPathTask {
    fn name(&self) -> String {
        format!("set_path_task")
    }

    fn execute(&self) -> Result<()> {
        let powershell_string = powershell_string(self);
        let acceptable_status_codes = [1];
        process::execute(
            "powershell.exe",
            &["-command", &powershell_string],
            &acceptable_status_codes,
        )
        .wrap_err_with(|| format!("Executing powershell string '{powershell_string}'"))
    }
}

fn powershell_string(_task: &SetPathTask) -> String {
    format!(
        r#"
        $env:Path += ";$home\bin"
        [Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::User)
    "#
    )
}

pub(crate) fn set_path_task() -> Box<dyn task::Task> {
    Box::new(SetPathTask)
}
