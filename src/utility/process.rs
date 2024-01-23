use color_eyre::{
    eyre::{Context, Ok, Report},
    Result,
};
use std::process::{Command, Stdio};

pub fn execute(program: &str, args: &[&str], acceptable_status_codes: &[i32]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .wrap_err("Executing command")?;
    let code = status.code().unwrap_or(0);

    println!("Status code: {code:?}");
    // -1978335189 = 0x8a15002b = winget nothing to upgrade
    // -1978335226 = 0x8a150006 = winget could not install (because package in already installed. Problems with ProtonVPN)
    // -1978335212 = 0x8a150014 = winget failed to find package
    // -1978335215 = 0x8a150011 = winget installer hash does not match
    if status.success() || acceptable_status_codes.contains(&code) {
        Ok(())
    } else {
        Err(Report::msg(format!(
            "Non-zero error code for command: '{status}'"
        )))
    }
}
