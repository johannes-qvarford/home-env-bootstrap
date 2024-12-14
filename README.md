# home-env-bootstrap

* Download the latest release from https://github.com/johannes-qvarford/home-env-bootstrap/releases/latest
* Open an admin powershell window in the Downloads directory.
* Run ./boostrap.exe
* Download and run pulseaudio-win32 full installer from https://pgaskin.net/pulseaudio-win32/
* May need to create an inbound/outbound firewall rule for port 45789

* Try running https://www.microsoft.com/p/app-installer/9nblggh4nns1 if winget cannot connect


# Stuff

rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64 lld

cargo build --target x86_64-pc-windows-gnu --release
./target/x86_64-pc-windows-gnu/debug/bootstrap.exe --task install_wsl