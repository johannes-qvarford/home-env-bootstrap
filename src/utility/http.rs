use bytes::Bytes;

use color_eyre::{eyre::{Context, Report}, Result};

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