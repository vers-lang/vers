#!/bin/bash

echo "Setting up root directory for Vers..."
sudo mkdir /lib/vers/
sudo cp -r vers/version /lib/vers/
sudo cp -r vers/install /lib/vers/
echo "Compiling Vers commands"
pyinstaller cli/src/new.py --onefile
echo "Move Vers binary"
mv dist/new /bin/vers_new
echo "Compiling compiler..."
cargo build
sudo mv target/debug/vers /bin/
