#!/bin/bash
set -euo pipefail

# install libclang-dev
sudo apt update -y
sudo apt install -y curl libclang-dev lsb-release

# install groonga
DEB_PKG="groonga-apt-source-latest-$(lsb_release --codename --short).deb"
curl -sSLO --output-dir /tmp "https://packages.groonga.org/ubuntu/${DEB_PKG}"
sudo apt-get install -y "/tmp/${DEB_PKG}"
sudo apt update -y

sudo apt-get install -y \
    groonga \
    libgroonga-dev
