use color_eyre::{eyre::Context, Result};

use crate::{
    utility::process,
    utility::task::{self},
};

pub(crate) struct MoveSensitiveInformation;

impl task::Task for MoveSensitiveInformation {
    fn name(&self) -> String {
        "move_sensitive_information".to_owned()
    }

    fn execute(&self) -> Result<()> {
        let powershell_string = powershell_string(self);
        process::execute("powershell.exe", &["-command", &powershell_string], &[])
            .wrap_err_with(|| format!("Executing powershell string '{powershell_string}'"))
    }
}

fn powershell_string(_task: &MoveSensitiveInformation) -> String {
    // TODO: Break out into individual tasks for each directory
    // TODO: Do everything is Rust.
    // TODO: Do not use the Z: drive if it causes problems
    // TODO: In fact, just get a 1TB boot drive and encrypt it.
    r#"
    function Move-AppData {
        param (
            $Directory
        )
        $SourceDirectory = "$env:HOMEPATH\AppData\$Directory"
        $Item = Get-Item $SourceDirectory
        $LinkExists = $Item.Attributes -band [System.IO.FileAttributes]::ReparsePoint
    
        if ($LinkExists) {
            Write-Output "Link already exists for $SourceDirectory"
            return
        }
    
        $DestinationDirectory = "Z:\$Directory"
        Move-Item -Path "$SourceDirectory" -Destination "$DestinationDirectory"
        New-Item -Path "$SourceDirectory" -ItemType SymbolicLink -Value "$DestinationDirectory"
    }

    cd C:
    
    New-Item -ItemType Directory Z:\Secrets
    New-Item -ItemType Directory Z:\Local
    New-Item -ItemType Directory Z:\Roaming
    
    Move-AppData -Directory "Roaming\Mozilla"
    Move-AppData -Directory "Roaming\Bitwarden"
    Move-AppData -Directory "Local\Mozilla"
    "#
    .to_owned()
}

pub(crate) fn move_sensitive_information() -> Box<dyn task::Task> {
    Box::new(MoveSensitiveInformation)
}
