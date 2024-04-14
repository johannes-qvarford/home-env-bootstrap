cargo build --target x86_64-pc-windows-gnu --release
mv ./target/x86_64-pc-windows-gnu/release/bootstrap.exe /mnt/c/Users/deffo/
powershell.exe -Command '& $env:USERPROFILE\bootstrap.exe '"$@"