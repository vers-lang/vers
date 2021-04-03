#include <iostream>

#include "../clib/messages.h"
#include "compiler.h"

using namespace std;

int compiler_main() {
    setup_project();
    translate_vers();
    return 0;
}
