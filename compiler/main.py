from compiler.compile import *
from compiler.errors import *
from compiler.finish import *
from compiler.vers import *

from colorama import *
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
    print(f"{Fore.LIGHTGREEN_EX}Compiling {Fore.BLUE}{project_file['name']} {Fore.GREEN}{project_file['version']}{Style.RESET_ALL}")
    setup()
    if os.path.exists("build/"):
        print("Cleaning...")
        os.system('rm -rf build/')
    print(f"Creating {Fore.BLUE}build/{Style.RESET_ALL} directory")
    build_dir()
    build_script()
    compile_vers(start_file)
    print(f"{Fore.LIGHTGREEN_EX}Compiling {Fore.BLUE}internal{Style.RESET_ALL} files...")
    compile_internal()
    print(f"{Fore.LIGHTGREEN_EX}Compiling {Fore.BLUE}external{Style.RESET_ALL} files...")
    compile_external()
    if start_file == "src/main.vers":
        link("")
    elif start_file == "src/lib.vers":
        link("-nostartfiles")
    finish()
    print(f"{Fore.GREEN}Done{Style.RESET_ALL}")
    print(f"\nDone {Fore.LIGHTGREEN_EX}compiling {Fore.BLUE}{project_file['name']}{Style.RESET_ALL}")
