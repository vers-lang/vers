#include "color.h"
#include <iostream>

#include "messages.h"


using namespace std;

string compiler_warning(int line, string msg, string arg) {
    cout << yellow << line << ": " << msg << arg << clear << endl;
    return msg;
}

string compiler_error(int line, string msg, string arg) {
    cout << red << line << ": " << msg << arg << clear << endl;
    return msg;
}

string compiler_message(string message) {
    cout << green << message << clear << endl;
    return message;
}
