#include <boost/algorithm/string.hpp>
#include "../../clib/string_man.h"
#include <iostream>

using namespace boost;
using namespace std;

string create_function(string line) {
    replace_str(line, "fun", "()", "{", "");
    trim(line);
    line + ":";
    return line;
}

