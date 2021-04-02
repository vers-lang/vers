#include <iostream>
#include <string.h>

using namespace std;

extern "C++" int compiler_main();

int main(int argc, char *argv[]) {
    if (!strcmp(argv[1], "build")) {
        compiler_main();
    } else {
        cout << "Argument isn't test" << endl;
    }
}
