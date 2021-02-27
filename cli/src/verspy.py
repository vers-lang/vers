import sys
import cli.src.install
import cli.src.new


def main(arg):
    if arg == "new":
        cli.src.new.main()
    elif arg == "install":
        cli.src.install.main()


if __name__ == '__main__':
    arg = sys.argv[0]
    main(arg)
