### Build Script

A build script in a file that runs when your project is compiling, this file can compile external files like C or ASM 
files. For Vers to run you build script you need to add a file named ``build.sh`` in the project directory not the 
source(``src/``) directory.

Example:
```shell
#!/bin/bash
# build.sh

cp -r src/main.c build/external/main.c
```
