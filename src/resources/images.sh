#!/bin/bash

cd $(mktemp -d)
name=magick

curl -L https://imagemagick.org/archive/binaries/$name -O $name
chmod 755 ./$name
sudo mv $name /usr/local/bin/

name=Inkscape-091e20e-x86_64.AppImage

curl -L https://inkscape.org/gallery/item/44616/$name -O $name
chmod 755 ./$name
sudo mv $name /usr/local/bin/inkscape