use color_eyre::{eyre::Context, Result};

use crate::{utility::process, utility::task};

const WINGET_INSTALLER_HASH_DOES_NOT_MATCH: i32 = -1978335215;
const _WINGET_FAILED_TO_FIND_PACKAGE: i32 = -1978335212;
const WINGET_FAILED_TO_INSTALL_PACKAGE: i32 = -1978335226;
const WINGET_PACKAGE_CANNOT_BE_UPGRADED: i32 = -1978335189;

pub(crate) struct Winget {
    pub(crate) package_name: &'static str,
}

impl task::Task for Winget {
    fn name(&self) -> String {
        let package_name = self.package_name;
        format!("winget_install_{package_name}")
    }

    fn execute(&self) -> Result<()> {
        let package_name = self.package_name;
        let acceptable_status_codes = [
            WINGET_INSTALLER_HASH_DOES_NOT_MATCH,
            WINGET_FAILED_TO_INSTALL_PACKAGE,
            WINGET_PACKAGE_CANNOT_BE_UPGRADED,
        ];
        process::execute(
            "winget.exe",
            &["install", self.package_name],
            &acceptable_status_codes,
        )
        .wrap_err(format!("Calling winget install {package_name}"))
    }
}

pub(crate) fn winget_task(package_name: &'static str) -> Box<dyn task::Task> {
    Box::new(Winget { package_name })
}
