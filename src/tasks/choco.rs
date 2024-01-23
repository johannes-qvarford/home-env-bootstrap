use color_eyre::{eyre::Context, Result};

use crate::{utility::process, utility::task};

pub(crate) struct Choco {
    pub(crate) package_name: &'static str,
}

impl task::Task for Choco {
    fn name(&self) -> String {
        let package_name = self.package_name;
        format!("choco_install_{package_name}")
    }

    fn execute(&self) -> Result<()> {
        let package_name = self.package_name;
        process::execute("choco.exe", &["install", "-y", self.package_name], &[])
            .wrap_err(format!("Calling choco install {package_name}"))
    }
}

pub(crate) fn choco(package_name: &'static str) -> Box<dyn task::Task> {
    Box::new(Choco { package_name })
}
