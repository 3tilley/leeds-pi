#!/bin/bash

set -eu

readonly new_host=pi@rpi1-v12b

echo "=== Copying SSH public key ==="
ssh-copy-id -i ./leedspi_rsa.pub $new_host

echo "=== Testing Internet Connectivity ==="
ssh -i ./leedspi_rsa -t $new_host "ping -c4 8.8.8.8"

echo "=== Testing DNS ==="
ssh -i ./leedspi_rsa -t $new_host "ping -c4 www.google.com"

echo "=== Add .bashrc ==="
scp -i ./leedspi_rsa ./.bashrc $new_host:~/.bashrc






