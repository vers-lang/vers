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


if __name__ == '__main__':
    arg1 = sys.argv[1]
    main(arg1)
