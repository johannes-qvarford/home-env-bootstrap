# home-env-bootstrap

* Download the latest release from https://github.com/johannes-qvarford/home-env-bootstrap/releases/latest
* Open an admin powershell window in the Downloads directory.
* Run ./boostrap.exe


* Try running https://www.microsoft.com/p/app-installer/9nblggh4nns1 if winget cannot connect


# Stuff

rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64

cargo build --target x86_64-pc-windows-gnu --release
./target/x86_64-pc-windows-gnu/debug/bootstrap.exe --task install_wsl