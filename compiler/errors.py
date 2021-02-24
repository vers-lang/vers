from colorama import *
import sys


def compiler_error(message):
    print(f"{Fore.RED}{message}{Style.RESET_ALL}")
    sys.exit(0)


# Vers errors
e1v = "E1V: Couldn't read project.json file"
e2v = "E2V: Unknown instruction"

# Hardware errors
e1h = "E1H: CPU architecture not supported yet"
