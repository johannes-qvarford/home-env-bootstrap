use color_eyre::{eyre::Context, Result};

use crate::{
    utility::process,
    utility::task::{self},
};

pub(crate) struct ScheduledTask {
    name: String,
    at: String,
}

impl task::Task for ScheduledTask {
    fn name(&self) -> String {
        let ScheduledTask { name, .. } = self;
        format!("scheduled_task_{name}")
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

fn powershell_string(task: &ScheduledTask) -> String {
    let ScheduledTask { name, at } = task;

    format!(
        r#"
        $name = "{name}"
        $at = "{at}"
        $exists = (Get-ScheduledTask | Where-Object {{ $_.TaskName -eq $name }}).Count -ne 0
        if ($exists) {{
            return;
        }}
        $trigger = New-ScheduledTaskTrigger -Weekly -DaysOfWeek Sunday -At $at
        $action = New-ScheduledTaskAction -Execute 'C:\WINDOWS\system32\wsl.exe' -Argument "/home/jq/home-env/schedule/$name"
        
        Register-ScheduledTask -Action $action -Trigger $trigger -TaskName $name -Description $name -TaskPath "\Schedule\"
    "#
    )
}

pub(crate) fn scheduled_task_task(name: &str, at: &str) -> Box<dyn task::Task> {
    Box::new(ScheduledTask {
        name: name.to_owned(),
        at: at.to_owned(),
    })
}
