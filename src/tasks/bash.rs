use color_eyre::{eyre::Context, Result};

use crate::{
    utility::process,
    utility::task::{self},
};

pub(crate) struct Bash {
    name: &'static str,
    content: &'static str,
}

impl task::Task for Bash {
    fn name(&self) -> String {
        let Bash { name, .. } = self;
        format!("bash_{name}")
    }

    fn execute(&self) -> Result<()> {
        let Bash { content, .. } = self;

        let complete_content = format!(
            r#"
        export PATH="$HOME/bin:$HOME/.cargo/bin:$PATH"
        {content}
        "#
        );

        process::execute("wsl.exe", &["-e", "bash", "-c", &complete_content], &[])
            .wrap_err_with(|| format!("Executing bash string '{complete_content}'"))
    }
}

pub(crate) fn bash(name: &'static str, content: &'static str) -> Box<dyn task::Task> {
    Box::new(Bash { name, content })
}

#[macro_export]
macro_rules! bash {
    ($file:expr) => {{
        tasks::bash::bash($file, include_str!(concat!("resources/", $file, ".sh")))
    }};
}
