#!/bin/bash

cargo clean
rm -rf build/
rm -rf dist/
rm -rf verspy.spec
echo "Setting up root directory for Vers..."
sudo mkdir /lib/vers/
sudo cp -r vers/version /lib/vers/
sudo cp -r vers/install /lib/vers/
mkdir /home/$USER/.vers/
echo "Compiling Vers commands"
pyinstaller cli/src/verspy.py --onefile
echo "Move Vers binary"
sudo mv dist/verspy /bin/verspy
echo "Compiling compiler..."
cargo build
sudo mv target/debug/vers /bin/
