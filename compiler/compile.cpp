#include "versl/asm.h"
#include "../clib/messages.h"
#include "compiler.h"

using namespace std;

int compiler_main() {
    setup_project();
    translate_vers();
    compile_vers();
    return 0;
}
