#!/bin/bash

echo "Compiling Vers commands and compiler..."
sleep 1
pyinstaller vers.py --onefile
mv dist/vers /bin/vers
# echo "Compiling Vers stdlib..."
# cd std/ && mkdir /lib/vers/lib/
# vers build
# mv build/std /lib/vers/lib/
