use crate::utility::task;



#[cfg(unix)]
pub(crate) fn tasks() -> Vec<Box<dyn task::Task>> {
    //use crate::linux_tasks as t;
    use crate::bash_task;
    vec![
        bash_task!("test"),
        // // Clone repository
        bash_task!("start"),
        bash_task!("github"),
        bash_task!("clone-home-env"),
        // Home-env usage.
        bash_task!("dotfiles"),
        bash_task!("fish"),
        // win-server
        bash_task!("wslu"),
        bash_task!("docker"),
        // privacy
        bash_task!("ansible"),
        // languages
        bash_task!("nushell"),
        bash_task!("powershell"),
        bash_task!("rust"),
        bash_task!("java"),
        bash_task!("zig"),
        // Fun
        bash_task!("images"),
        bash_task!("httpie"),
        bash_task!("colors"),
        bash_task!("fzf"),
        bash_task!("extra"),
        bash_task!("backup"),
    ]
}

#[cfg(windows)]
pub(crate) fn tasks() -> Vec<Box<dyn task::Task>> {
    use crate::windows_tasks as t;

    vec![
        // No restart is needed nowadays.
        t::install_wsl_task(),
        // Development
        t::winget_task("Microsoft.WindowsTerminal"),
        t::winget_task("Microsoft.VisualStudioCode"),
        t::winget_task("gerardog.gsudo"),
        t::winget_task("9PGCV4V3BK4W"), // DevToys
        t::winget_task("DBBrowserForSQLite.DBBrowserForSQLite"),
        t::winget_task("Microsoft.PowerToys"),

        // Media
        t::winget_task("Mozilla.Firefox"),
        t::winget_task("Brave.Brave"),
        t::winget_task("Valve.Steam"),
        t::winget_task("XBMCFoundation.Kodi"),
        t::winget_task("VideoLAN.VLC"),
        t::winget_task("Chocolatey.Chocolatey"),
        t::choco_task("tartube"),
        // Privacy
        t::winget_task("Proton.ProtonDrive"),
        t::winget_task("ProtonTechnologies.ProtonVPN"),
        t::winget_task("Bitwarden.Bitwarden"),
        // Utility
        t::winget_task("WinDirStat.WinDirStat"),
        t::winget_task("BleachBit.BleachBit"),
        t::winget_task("7zip.7zip"),
        t::winget_task("HandBrake.HandBrake"),
        t::choco_task("messenger"),
        // WSL
        t::scheduled_task_task("backup-media", "11:00 am"),
        t::scheduled_task_task("upgrade-tools", "12:00 pm"),
        t::connect_windows_terminal_task(),

        t::download_bootstrap_linux_task(),
        t::run_bootstrap_linux_task()
    ]
}