#!/bin/bash

reg.exe add 'HKEY_CLASSES_ROOT\Directory\shell\OpenInWSL' /d 'Open in WSL'
reg.exe add 'HKEY_CLASSES_ROOT\Directory\shell\OpenInWSL' /v Icon /d 'C:\WINDOWS\system32\wsl.exe'
reg.exe add 'HKEY_CLASSES_ROOT\Directory\shell\OpenInWSL\command2' /d 'wsl.exe --cd "%V"'