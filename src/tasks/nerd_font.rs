use std::{
    fs::{self, File},
    io::Write,
};

use color_eyre::{
    eyre::{Context, ContextCompat, Report},
    Result,
};

use crate::utility::task::{self};

pub(crate) struct NerdFont;

impl task::Task for NerdFont {
    fn name(&self) -> String {
        "nerd_font".to_owned()
    }

    fn execute(&self) -> Result<()> {
        let mut fonts_dir =
            dirs::config_local_dir().wrap_err("Locating local configuration directory")?;
        fonts_dir.push("Microsoft/Windows/Fonts");
        let fonts_dir_clone = fonts_dir.clone();
        fs::create_dir_all(&fonts_dir).wrap_err_with(|| {
            format!("Creating fonts directory if it does not exist: '{fonts_dir_clone:?}'")
        })?;

        let font_url = "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/JetBrainsMonoNerdFontMono-Regular.ttf";
        let font_response = reqwest::blocking::get(font_url)
            .wrap_err_with(|| format!("Fetching Nerd Font from: {font_url}"))?;
        let status = font_response.status();

        if !status.is_success() {
            return Err(Report::msg(format!(
                "Unsuccessful response when fetching font url: {status}"
            )));
        }

        let bytes = font_response
            .bytes()
            .wrap_err("Converting response to bytes")?;

        let mut font_path = fonts_dir.clone();
        font_path.push("JetbrainsMono NF.ttf");
        let mut file = File::create(font_path)
            .wrap_err_with(|| format!("Creating font file at '{fonts_dir_clone:?}'"))?;
        file.write_all(&bytes)
            .wrap_err("Writing bytes to font file")?;

        Ok(())
    }
}

pub(crate) fn nerd_font() -> Box<dyn task::Task> {
    Box::new(NerdFont)
}
