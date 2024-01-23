#!/bin/bash

sudo apt-add-repository -y ppa:fish-shell/release-3
sudo apt update

sudo apt install -y fish
chsh -s /usr/bin/fish
fish -c "curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source && fisher install jorgebucaran/fisher"