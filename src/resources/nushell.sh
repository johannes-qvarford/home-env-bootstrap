#!/bin/bash

version=$(gh-release-latest nushell/nushell)
tar_dir="nu-$version-x86_64-unknown-linux-gnu"
tar_file="$tar_dir.tar.gz"

cd "$(mktemp -d)"
gh-release-latest-download nushell/nushell "$tar_file"
extract "$tar_file"
sudo mv "$tar_dir/nu" /usr/local/bin