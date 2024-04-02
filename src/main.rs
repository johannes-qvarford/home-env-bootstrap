#![windows_subsystem = "console"]


use color_eyre::eyre::ContextCompat;
use color_eyre::{eyre::Context, Result};

// TODO: Separate utility from tasks
mod tasks;
mod utility;

use colored::Colorize;
use tasks::choco::choco;
use tasks::connect_windows_terminal::connect_windows_terminal;
use tasks::install_wsl::install_wsl;
use tasks::nerd_font::nerd_font;
use tasks::scheduled_task::scheduled_task;
use tasks::winget::winget;

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

    let tasks = vec![
        // No restart is needed nowadays.
        install_wsl(),
        // Development
        winget("Microsoft.WindowsTerminal"),
        winget("Microsoft.VisualStudioCode"),
        winget("gerardog.gsudo"),
        winget("9PGCV4V3BK4W"), // DevToys
        winget("DBBrowserForSQLite.DBBrowserForSQLite"),
        winget("Microsoft.PowerToys"),

        // Media
        winget("Mozilla.Firefox"),
        winget("Brave.Brave"),
        winget("Valve.Steam"),
        winget("XBMCFoundation.Kodi"),
        winget("VideoLAN.VLC"),
        winget("Chocolatey.Chocolatey"),
        choco("tartube"),
        // Privacy
        winget("Proton.ProtonDrive"),
        winget("ProtonTechnologies.ProtonVPN"),
        winget("Bitwarden.Bitwarden"),
        // Utility
        winget("Facebook.Messenger"),
        winget("WinDirStat.WinDirStat"),
        winget("BleachBit.BleachBit"),
        winget("7zip.7zip"),
        winget("HandBrake.HandBrake"),
        winget("testy"),
        // WSL

        // Clone repository
        bash!("start"),
        bash!("github"),
        bash!("clone-home-env"),
        // Home-env usage.
        bash!("dotfiles"),
        bash!("fish"),
        // win-server
        bash!("wslu"),
        bash!("systemd"),
        bash!("docker"),
        // privacy
        bash!("ansible"),
        // languages
        bash!("nushell"),
        bash!("powershell"),
        bash!("rust"),
        bash!("java"),
        bash!("zig"),
        // Fun
        bash!("httpie"),
        bash!("colors"),
        bash!("fzf"),
        bash!("extra"),
        bash!("backup"),
        // Not needed, but keep it around for reference
        // bash!("wsl-context-menu-item"),
        scheduled_task("backup-media", "11:00 am"),
        scheduled_task("upgrade-tools", "12:00 pm"),
        // Cool fonts
        nerd_font(),
        connect_windows_terminal(),
    ];

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
