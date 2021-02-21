#!/bin/bash

echo "Compiling Vers commands and compiler..."
sleep 1
pyinstaller vers.py --onefile
mv dist/vers /bin/vers
echo "Compiling Vers stdlib..."
mkdir /lib/vers/ && mkdir /lib/vers/lib
cd std && vers build
mv build/libstd /lib/vers/lib/
