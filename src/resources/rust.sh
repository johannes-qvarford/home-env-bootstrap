#!/bin/bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
sudo apt install -y build-essential pkg-config libssl-dev
rustup component add rustfmt