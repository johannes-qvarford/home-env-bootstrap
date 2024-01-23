#!/bin/bash

sudo apt install -y git

if [[ ! -f ~/.ssh/id_rsa.pub ]]; then
    ssh-keygen -t rsa -b 2048 -C "jq.email+github@pm.me"
    echo
    echo paste the below into https://github.com/settings/keys and 
    echo
    cat ~/.ssh/id_rsa.pub
    echo
    read -p "Have you copied the above? "
else
    echo Skipping generation of id_rsa pair
fi

if [[ ! -f ~/.ssh/id_rsa_privacy.pub ]]; then
    ssh-keygen -t rsa -b 2048 -C "jq.email+github@pm.me" -f ~/.ssh/id_rsa_privacy
    echo
    echo paste the below into https://cloud.digitalocean.com/account/security
    echo 
    cat ~/.ssh/id_rsa_privacy.pub
    echo
    echo now a passphrase will be requested.
    ssh-keygen -p -o -f ~/.ssh/id_rsa_privacy
else
    echo Skipping generation of id_rsa_privacy pair
fi

type -p curl >/dev/null || (sudo apt update && sudo apt install curl -y)
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
&& sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
&& sudo apt update \
&& sudo apt install gh -y

gh auth login