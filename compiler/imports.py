import compiler.vers
from compiler.warnings import *

from colorama import *
import os


def add_import(lib, name):
    if os.path.exists(f"/home/$USER/verslib/{name}"):
        os.system(f'cp -r {lib} /home/$USER/verslib/lib/')
        print(f"    {Fore.LIGHTGREEN_EX}Adding {Fore.BLUE}{name} to project...{Style.RESET_ALL}")
        os.system(f'cp -r /home/$USER/verslib/lib/{lib} build/imports/')
    else:
        compiler_warning(w1v)


def compile_import(name):
    print(f"    {Fore.LIGHTGREEN_EX}Compiling imported library: {Fore.BLUE}{name}...{Style.RESET_ALL}")
    print("    ---------------------------")
    os.system(f'cd /home/$USER/verslib/{name}/ && vers build')
    print(f"    Done compiling {Fore.BLUE}{name}{Style.RESET_ALL}")
    add_import(f"/home/$USER/verslib/{name}/build/lib{name}", name)

