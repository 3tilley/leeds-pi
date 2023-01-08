#!/bin/bash

set -eu

readonly new_host=pi@rpi2-v4b

echo "=== Copying SSH public key ==="
ssh-copy-id -i ./leedspi_rsa.pub $new_host

echo "=== Testing Internet Connectivity ==="
ssh -t $new_host "ping -c4 8.8.8.8"

echo "=== Testing DNS ==="
ssh -t $new_host "ping -c4 www.google.com"

echo "=== Add configs ==="
scp ./.bashrc $new_host:~/.bashrc
scp ./.inputrc $new_host:~/.inputrc
scp ./gritconfig $new_host:~/.gitconfig
#vimrc

echo "=== Create folders ==="
ssh -t $new_host "mkdir -p ~/leeds-pi"
ssh -t $new_host "mkdir -p ~/git"

# Add libgpiod2
# Add raspi config to enable SPI







