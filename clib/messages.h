#ifndef VERS_MESSAGES_H
#define VERS_MESSAGES_H

#include <iostream>

using namespace std;

extern "C++" string compiler_error(int line, string msg, string arg);
extern "C++" string compiler_warning(int line, string msg, string arg);
extern "C++" string compiler_message(string message);

#endif //VERS_MESSAGES_H
