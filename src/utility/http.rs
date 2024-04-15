#[cfg(windows)]
use bytes::Bytes;
#[cfg(windows)]
use color_eyre::{eyre::{Context, Report}, Result};

#[cfg(windows)]
pub(crate) fn download(url: &str) -> Result<Bytes> {
    let response = reqwest::blocking::get(url)
        .wrap_err_with(|| format!("Downloading resource from: {url}"))?;
    let status = response.status();

    if !status.is_success() {
        return Err(Report::msg(format!(
            "Unsuccessful response when downloading resource from: {status}"
        )));
    }

    let bytes = response
        .bytes()
        .wrap_err("Extracting downloaded resource bytes")?;
    Ok(bytes)
}