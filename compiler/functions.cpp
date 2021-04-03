#include <boost/algorithm/string.hpp>
#include <iostream>

using namespace boost;
using namespace std;

string il_asm(string line) {
    boost::erase_all(line, "il_asm(\"");
    boost::erase_all(line, "\");");
    return line;
}
