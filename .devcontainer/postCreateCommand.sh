#!/usr/bin/env bash

set -e

sudo apt update
sudo apt install -y python3-pip

pip install -r .devcontainer/requirements.txt  --break-system-packages
