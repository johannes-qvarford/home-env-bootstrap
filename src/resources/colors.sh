#!/bin/bash

install () {
    repo="$1"
    pattern="$2"
    cd $(mktemp -d)
    gh-release-latest-download "$repo" "$pattern"
    sudo dpkg -i *
    sudo apt install -f
}

install Peltoche/lsd "lsd_*_amd64.deb"
install dandavison/delta "git-delta_*_amd64.deb"
sudo apt install -y fd-find bat