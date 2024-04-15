cargo build --release || exit 1
cp target/release/bootstrap /home/jq || exit 1
cargo build --target x86_64-pc-windows-gnu --release || exit 1
cp target/x86_64-pc-windows-gnu/release/bootstrap.exe /mnt/c/Users/deffo || exit 1

powershell.exe -Command '& $env:USERPROFILE\bootstrap.exe '"$@"