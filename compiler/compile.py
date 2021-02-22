import os

from colorama import *


def compile_internal():
    print(f"{Fore.LIGHTGREEN_EX}Compiling {Fore.BLUE}internal{Style.RESET_ALL} files...")
    os.system('cd build/internal && gcc -c main.S')


def compile_external():
    print(f"{Fore.LIGHTGREEN_EX}Compiling {Fore.BLUE}external{Style.RESET_ALL} files...")
    os.system('cd build/external && gcc -c *.c')
