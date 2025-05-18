sudo -E apt update && sudo apt upgrade -y \
    && sudo -E apt-get -qq install docker.io -y \
    && sudo apt install docker-compose \
    && echo $USER' ALL=(ALL) NOPASSWD: /usr/bin/dockerd' | sudo EDITOR='tee -a' visudo \
    && sudo usermod -aG docker $USER
# Need to shut down WSL for this to take effect
# Source: https://stackoverflow.com/a/70934691

# Needed for NVIDIA GPU support for AI models
# https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
  && curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
    sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
    sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list \
    && sudo apt-get update \
    && sudo apt-get install -y nvidia-container-toolkit