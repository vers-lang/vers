import os
import sys

from compiler.errors import *

last_known_word = ""


def compile_word(word):
    global last_known_word
    asm_file = open("build/internal/main.S", "a")
    if word == "decfun":
        asm_file.writelines(".globl ")
        last_known_word = "decfun"
    elif word == "decextern":
        asm_file.writelines(".extern ")
        last_known_word = "decextern"
    elif word == "fun":
        last_known_word = "fun"
    elif word == "asm":
        last_known_word = "asm"
    elif word == "end_asm":
        last_known_word = "end_asm"
    elif word == "at":
        asm_file.writelines("   ")
    elif word == "al":
        asm_file.writelines("\n")
    elif word == "/*":
        last_known_word = "/*"
    elif word == "*/":
        last_known_word = " "
    elif word == "{":
        last_known_word = word
    elif word == "}":
        last_known_word = word
    else:
        if last_known_word == "decfun":
            asm_file.writelines(f"{word}\n")
            last_known_word = word
        elif last_known_word == "decextern":
            asm_file.writelines(f"{word}\n")
            last_known_word = word
        elif last_known_word == "fun":
            fun_name = word.replace("()", "")
            asm_file.writelines(f"\n{fun_name}:\n")
            last_known_word = word
        elif last_known_word == "asm":
            asm_file.writelines(f"{word} ")
        elif last_known_word == "end_asm":
            last_known_word = " "
        elif last_known_word == "/*":
            pass
        else:
            if word.find("();"):
                call_fun = word.replace("();", "")
                asm_file.writelines(f"   call {call_fun}")
                last_known_word = word
            else:
                print(f"\n{word}")
                compiler_error(e1v)


def build_script():
    if os.path.exists("build.sh"):
        os.system('sh build.sh')
    else:
        pass


def compile_vers(file):
    print("Compiling internal files...")
    with open(file) as vers_file:
        for line in vers_file:
            for word in line.split():
                compile_word(word)


def build_dir():
    print("Creating build directory")
    os.mkdir("build/")
    os.mkdir("build/external/")
    os.mkdir("build/imports/")
    os.mkdir("build/internal/")
    os.system("touch build/internal/main.S")
