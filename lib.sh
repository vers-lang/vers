#!/bin/bash

echo "Compiling Vers stdlib..."
mkdir /lib/vers/ && mkdir /lib/vers/lib/
cp -r std/ /lib/vers/lib/
cd std/ && vers build
mv build/libstd /lib/vers/lib/
