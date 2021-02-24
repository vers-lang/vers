import cli.install
import cli.new
import cli.run
import cli.version

import json


def version():
    print("Vers ", cli.version.current_version())
    return cli.version.version


def new():
    print("Creating new project...")
    cli.new.create_new()
    print("Created new project")


def run():
    open_project_file = open("project.json").read()
    project_file = json.loads(open_project_file)
    if project_file['type'] == "exe":
        cli.run.run_project(project_file['name'])
    else:
        print("Not executable")


def install():
    lib = input("Library name: ")
    author = input("Author of library: ")
    libversion = input("Library version name: ")
    cli.install.install_lib(author, lib, libversion)
