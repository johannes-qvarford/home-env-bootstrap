#![windows_subsystem = "console"]


use color_eyre::eyre::ContextCompat;
use color_eyre::{eyre::Context, Result};

mod platform;
mod windows_tasks;
mod utility;

use colored::Colorize;
use clap::Parser;

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
    color_eyre::install()?;

    let args = Args::parse();
    let tasks = platform::tasks();
    
    if let Some(task_name) = args.task {
        let task = tasks.into_iter().find(|t| t.name() == task_name).with_context(|| format!("Looking for task with name {task_name}"))?;

        task.execute().wrap_err("Executing task")?;
        task.mark_executed().wrap_err("Marking task as executed")?;
        println!("{}", &format!("Forcefully executed task '{task_name}'.").yellow());
        return Ok(());
    }


    let mut skip = args.skip;
    for task in tasks {
        let name = task.name();

        if skip > 0 && !task.has_been_executed().wrap_err_with(|| format!("checking if task {name} has been executed"))? {
            skip -= 1;
            task.mark_executed().wrap_err_with(|| format!("marking task {name} as executed"))?;
            println!("{}", &format!("Forcefully marked task '{name}' as executed.").yellow());
        }

        task
            .execute_if_needed()
            .wrap_err_with(|| format!("Executing task '{name}' if needed"))?;
    }

    println!("{}", "Done! Press any button!".green());
    std::io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
