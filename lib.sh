#!/bin/bash

echo "Compiling Vers stdlib..."
mkdir /home/$USER/verslib/ && /home/$USER/verslib/lib/
cp -r std/ /home/$USER/verslib/
cd std/ && vers build
mv build/libstd /home/$USER/verslib/lib
