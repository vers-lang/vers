#include "../../clib/messages.h"
#include "gcc.h"
#include <unistd.h>

void compile_exe() {
    chdir("build/internal/");
    object_file("main.S");
    system("gcc main.o -o ../main");
}
