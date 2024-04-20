use std::{
    fmt::Debug,
    fs::{create_dir_all, File},
    path::PathBuf,
};

use color_eyre::{
    eyre::{Context, Report},
    Result,
};

use owo_colors::{OwoColorize, Stream::Stdout};

use dirs::config_local_dir;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Execution {
    Performed,
    Skipped,
}

pub(crate) trait Task {
    fn name(&self) -> String;
    fn execute(&self) -> Result<()>;
    fn requires_restart(&self) -> bool {
        false
    }
}

impl dyn Task {
    pub(crate) fn execute_or_pause(&self) -> Result<()> {
        self.execute()
    }

    pub(crate) fn execute_if_needed(&self) -> Result<Execution> {
        let name = self.name();
        create_dir_all(self.mark_directory()?).wrap_err("Creating mark directory")?;
        let executed = self
            .has_been_executed()
            .wrap_err("Checking if task been executed")?;
        if !executed {
            self.execute_or_pause().wrap_err("Executing tasks")?;
            self.mark_executed().wrap_err("Marking task as executed")?;
            println!(
                "{}",
                &format!("Marked task '{name}' as executed.")
                    .if_supports_color(Stdout, |x| x.green())
            );
            Ok(Execution::Performed)
        } else {
            println!(
                "{}",
                &format!("Skipping task '{name}' since it has already been executed.")
                    .if_supports_color(Stdout, |x| x.blue())
            );
            Ok(Execution::Skipped)
        }
    }

    pub(crate) fn has_been_executed(&self) -> Result<bool> {
        let path = self.mark_path()?;
        path.try_exists()
            .wrap_err_with(|| format!("Verifying existance of mark path '{path:?}'"))
    }

    pub(crate) fn mark_executed(&self) -> Result<()> {
        File::create(
            self.mark_path()
                .wrap_err("Calculating path of mark file to create")?,
        )
        .wrap_err("Creating mark file")?;
        Ok(())
    }

    fn mark_path(&self) -> Result<PathBuf> {
        let mut path = self.mark_directory().wrap_err("Calculating mark path")?;
        path.push(&self.name());
        Ok(path)
    }

    fn mark_directory(&self) -> Result<PathBuf> {
        let mut dir =
            config_local_dir().ok_or(Report::msg("Config local directory was not found"))?;
        dir.push("home-env-bootstrap");
        Ok(dir)
    }
}
