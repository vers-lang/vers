#include "asm.h"
#include <bits/stdc++.h>
#include "../project.h"

using namespace std;

int compile_vers() {
    if (project_type == "exe") {
        compile_exe();
    } else if (project_type == "lib") {
        compile_lib();
    }
    return 0;
}
