#include "../clib/messages.h"
#include <iostream>
#include <fstream>
#include <unistd.h>
#include "project.h"
#include <string>
#include <sys/stat.h>

using namespace std;

int build_dir() {
    mkdir("build/", 0777);
    mkdir("build/internal/", 0777);
    mkdir("build/external/", 0777);
    ofstream asm_file("build/internal/main.S");
    asm_file.close();
    return 0;
}

int setup_project() {
    rmdir("build/");
    ifstream project_file("project.txt");
    string line;
    int line_num;
    if (project_file.is_open()) {
        while (getline(project_file, line)) {
            line_num = line_num + 1;
            if (line_num == 1) {
                project_name = line.erase(0, 6);
            } else if (line_num == 2) {
                project_type = line.erase(0, 6);
            }
        }
    }
    compiler_message("Compiling " + project_name + "...");
    compiler_message("    Creating build directory...");
    build_dir();
    return 0;
}
