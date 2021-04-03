#include "../arch/x86_64.h"
#include <boost/algorithm/string.hpp>
#include "../../clib/string_man.h"
#include "../../clib/messages.h"
#include <iostream>
#include <fstream>
#include "vers.h"

using namespace boost;
using namespace std;

int translate_vers() {
    compiler_message("    Translating Vers...");
    int line_num = 0;
    string vline;
    fstream asm_file("build/internal/main.S");
    asm_file << dec_main + "\n" << endl;
    fstream vers_file("src/main.vers");

    while (getline(vers_file, vline)) {
        line_num = line_num + 1;

        if (!vline.find("fun")) {
            // string function = create_function(vline);
            string function = replace_str(vline, "fun", "()", "{", "");
            trim(function);
            asm_file << function + fun + "\n" << endl;
        } else if (!vline.find("extern")) {
            string extern_function = replace_str(vline, "extern", ";", "", "");
            asm_file << external + extern_function << endl;
        }
    }
    return 0;
}

