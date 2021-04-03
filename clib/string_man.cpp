#include <bits/stdc++.h>
#include <iostream>
#include "string_man.h"

using namespace std;

string replace_str(string full_string, string replace1, string replace2, string replace3, string replace_with) {
    // Clear replace1 in full_string
    while (full_string.find(replace1) != string::npos) {
        full_string.replace(full_string.find(replace1), 3, replace_with);
    }

    // Clear replace2 in full_string
    while (full_string.find(replace2) != string::npos) {
        full_string.replace(full_string.find(replace2), 3, replace_with);
    }

    // Clear replace3 in full_string
    while (full_string.find(replace3) != string::npos) {
        full_string.replace(full_string.find(replace3), 3, replace_with);
    }
    return full_string;
}

string replace_str4(string full_string, string replace1, string replace2, string replace3, char replace4, string replace_with) {
    // Clear replace1 in full_string
    while (full_string.find(replace1) != string::npos) {
        full_string.replace(full_string.find(replace1), 3, replace_with);
    }

    // Clear replace2 in full_string
    while (full_string.find(replace2) != string::npos) {
        full_string.replace(full_string.find(replace2), 3, replace_with);
    }

    // Clear replace3 in full_string
    while (full_string.find(replace3) != string::npos) {
        full_string.replace(full_string.find(replace3), 3, replace_with);
    }

    // Clear replace3 in full_string
    while (full_string.find(replace4) != string::npos) {
        full_string.replace(full_string.find(replace4), 3, replace_with);
    }

    return full_string;
}
