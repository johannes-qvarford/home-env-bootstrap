#!/bin/bash

go () {
    gsudo.exe powershell.exe -Command "$@"
}

go 'New-Item "Registry::HKEY_CLASSES_ROOT\Directory\shell\OpenInWSL" -Value "Open in WSL"'
go 'New-ItemProperty "Registry::HKEY_CLASSES_ROOT\Directory\shell\OpenInWSL" -Name Icon -Value "C:\WINDOWS\system32\wsl.exe"'
go 'New-Item "Registry::HKEY_CLASSES_ROOT\Directory\shell\OpenInWSL\command" -Value '"'"'wsl.exe --cd "%V"'"'"''