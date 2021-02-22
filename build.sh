#!/bin/bash

echo "Compiling Vers commands and compiler..."
sleep 1
pyinstaller vers.py --onefile
mv dist/vers /bin/vers
