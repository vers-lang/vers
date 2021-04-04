#include <algorithm>
#include "../arch/x86_64.h"
#include <boost/algorithm/string.hpp>
#include "../../clib/string_man.h"
#include "../../clib/messages.h"
#include <iostream>
#include "../functions.h"
#include <fstream>

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

        boost::erase_all(vline, "}");

        if (!vline.find("fun")) {
            string function = replace_str(vline, "fun", "()", "{", "");
            trim(function);
            asm_file << function + fun << endl;
        } else if (!vline.find("extern")) {
            string extern_function = replace_str(vline, "extern", ";", "", "");
            asm_file << external + extern_function << endl;
        } /* else if (!vline.find("var")) {
            string variable = replace_str(vline, "var", "=", ";", "");
            variable = check_type(variable, line_num);
            asm_file << variable + "\n" << endl;
        } */ else if (vline.find("il_asm")) {
            asm_file << il_asm(vline) << endl;
        } else if (!vline.find("import")) {
            asm_file << "This is text and stuff \n ya" << endl;
        } else {
            compiler_error(line_num, "Unknown instruction\n", "Probably misspelled or doesn't exist");
        }
    }
    return 0;
}

