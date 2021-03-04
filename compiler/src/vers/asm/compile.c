#include <stdio.h>
#include <stdlib.h>

void compileInternal() {
    system("cd build/internal/ && gcc -c *.S");
}

void compileExternal() {
    system("cd build/externals/ && gcc -c *.c");
    system("cd build/external/ && gcc -c *.S");
}

// TODO: Compile and add imports
void addImports() {

}

void link(char filename) {

}