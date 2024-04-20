#!/bin/bash

sudo apt install libfuse2

curl -fsSL https://raw.githubusercontent.com/nagygergo/jetbrains-toolbox-install/master/jetbrains-toolbox.sh | bash

sudo cp ~/.config/icons/IntelliJ_IDEA_icon.svg /usr/share/icons/hicolor/scalable/apps/idea.svg
sudo cp ~/.config/icons/IntelliJ_IDEA_icon_512x512.png /usr/share/icons/hicolor/512x512/apps/idea.png

sudo cp ~/.config/icons/Toolbox_icon.svg /usr/share/icons/hicolor/scalable/apps/toolbox.svg
sudo cp ~/.config/icons/Toolbox_icon_512x512.png /usr/share/icons/hicolor/512x512/apps/toolbox.png

# This has trouble starting, probably related to how it uses the tray.
sudo tee /usr/share/applications/idea.desktop <<EOF
[Desktop Entry]
Name=IntelliJ IDEA
Icon=idea
Comment=IntelliJ IDEA
Exec="$HOME/.local/share/JetBrains/Toolbox/scripts/idea" %f
Version=1.0
Type=Application
Categories=Development;IDE;
Terminal=false
StartupNotify=true
EOF

sudo tee /usr/share/applications/jetbrains-toolbox.desktop <<EOF
[Desktop Entry]
Name=Jetbrains Toolbox
Icon=toolbox
Comment=Jetbrains Toolbox
Exec="$HOME/.local/bin/jetbrains-toolbox" %f
Version=1.0
Type=Application
Categories=Development;IDE;
Terminal=false
StartupNotify=true
EOF

# /usr/share/applications/org.gnome.gedit.desktop