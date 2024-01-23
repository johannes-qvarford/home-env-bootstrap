sudo -E apt update && sudo apt upgrade -y \
    && sudo -E apt-get -qq install docker.io -y \
    && sudo apt install docker-compose \
    && echo $USER' ALL=(ALL) NOPASSWD: /usr/bin/dockerd' | sudo EDITOR='tee -a' visudo \
    && sudo usermod -aG docker $USER
# Need to shut down WSL for this to take effect
# Source: https://stackoverflow.com/a/70934691