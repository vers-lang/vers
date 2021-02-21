from compiler.asm import *
from compiler.errors import *
from compiler.vers import *

import json
import os
import platform

start_file = " "


def setup():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    global start_file
    if project_file['type'] == "exe":
        start_file = "src/main.vers"
    elif project_file['type'] == "lib":
        start_file = "src/lib.vers"
    else:
        compiler_error(e1v)
    if platform.machine() == "x86" or platform.machine() == "x86_64":
        pass
    else:
        print(f"\n{platform.machine()}")
        compiler_error(e1h)


def main():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    print(f"Compiling {project_file['name']} {project_file['version']}")
    setup()
    if os.path.exists("build/"):
        print("Cleaning...")
        os.system('rm -rf build/')
    build_dir()
    build_script()
    compile_vers(start_file)
    compile_asm()
