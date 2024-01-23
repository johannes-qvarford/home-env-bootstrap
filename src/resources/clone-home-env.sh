#!/bin/bash

if [[ ! -d ~/home-env ]]; then
    gh repo clone johannes-qvarford/home-env ~/home-env
else
    echo "Skipping creation of home-env since it already exists"
fi

if [[ ! -d ~/home-env-bootstrap ]]; then
    gh repo clone johannes-qvarford/home-env-bootstrap ~/home-env-bootstrap
else
    echo "Skipping creation of home-env since it already exists"
fi