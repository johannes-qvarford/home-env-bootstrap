cargo build || exit 1
cp target/debug/bootstrap /home/jq || exit 1
cargo build --target x86_64-pc-windows-gnu || exit 1
cp target/x86_64-pc-windows-gnu/debug/bootstrap.exe /mnt/c/Users/deffo || exit 1

powershell.exe -Command '& $env:USERPROFILE\bootstrap.exe '"$@"