cargo build --target x86_64-pc-windows-gnu
mv ./target/x86_64-pc-windows-gnu/debug/bootstrap.exe /mnt/c/Users/deffo/
powershell.exe -Command '& $env:USERPROFILE\bootstrap.exe '"$@"