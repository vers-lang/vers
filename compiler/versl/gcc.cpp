#include <bits/stdc++.h>

int object_file(std::string file) {
    std::string obj = "gcc -c " + file;
    const char *command = obj.c_str();
    system(command);
    return 0;
}
