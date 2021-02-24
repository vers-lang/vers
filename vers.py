from cli.main import *
import compiler.main

import sys


def main(arg):
    if arg == "--version":
        version()
    elif arg == "--new":
        new()
    elif arg == "run":
        run()
    elif arg == "build":
        compiler.main.main()
    else:
        print(f"{arg} not a Vers command")


if __name__ == '__main__':
    arg1 = sys.argv[1]
    main(arg1)
