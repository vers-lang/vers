import sys


def compiler_error(message):
    print(message)
    sys.exit(0)


# Vers errors
e1v = "E1V: Couldn't read project.json file"

# Hardware errors
e1h = "E1H: CPU architecture not supported yet"
