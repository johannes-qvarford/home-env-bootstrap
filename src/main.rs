#![windows_subsystem = "console"]

use color_eyre::eyre::ContextCompat;
use color_eyre::{eyre::Context, Result};


mod platform;
#[cfg(unix)]
mod linux_tasks;
#[cfg(windows)]
mod windows_tasks;
mod utility;

use clap::Parser;
use owo_colors::{OwoColorize,Stream::Stdout};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Task to run exclusively
    #[arg(long)]
    task: Option<String>,

    #[arg(long, default_value_t = 0)]
    skip: i32
}

fn main() -> Result<()> {
    match private_main() {
        Ok(_) => {
            println!("{}", "Done! Press any button!".if_supports_color(Stdout, |x| x.green()));
            std::io::stdin().read_line(&mut String::new()).unwrap();
            std::process::exit(0)
        }
        Err(e) => {
            println!("{:?}", e);
            println!("{}", "No! Press any button!".if_supports_color(Stdout, |x| x.red()));
            std::io::stdin().read_line(&mut String::new()).unwrap();
            std::process::exit(1)
        },
    }
}

fn private_main() -> Result<()> {
    let no_colors = cfg!(windows) && cfg!(not(debug_assertions));
    std::env::set_var("RUST_SPANTRACE", "1");
    if no_colors {
        std::env::set_var("NO_COLOR", "1");
    } else {
        color_eyre::install()?;
    }
    
    let args = Args::parse();
    let tasks = platform::tasks();
    
    if let Some(task_name) = args.task {
        let task = tasks.into_iter().find(|t| t.name() == task_name).wrap_err_with(|| format!("Looking for task with name {task_name}"))?;
        task.execute_or_pause().wrap_err_with(|| format!("Executing task {task_name}"))?;
        task.mark_executed().wrap_err_with(|| format!("Marking task {task_name} as executed"))?;
        println!("{}", &format!("Forcefully executed task '{task_name}'.").if_supports_color(Stdout, |x| x.yellow()));
        return Ok(());
    }

    let mut skip = args.skip;
    for task in tasks {
        let name = task.name();

        if skip > 0 && !task.has_been_executed().wrap_err_with(|| format!("checking if task {name} has been executed"))? {
            skip -= 1;
            task.mark_executed().wrap_err_with(|| format!("marking task {name} as executed"))?;
            println!("{}", &format!("Forcefully marked task '{name}' as executed.").if_supports_color(Stdout, |x| x.yellow()));
        }

        task
            .execute_if_needed()
            .wrap_err_with(|| format!("Executing task '{name}' if needed"))?;
    }

    Ok(())
}
