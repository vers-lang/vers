from colorama import *
import json
import os


def link(start):
    print("Linking...")
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    os.system(f"cd build/ && gcc internal/*.o imports/* external/*.o -o internal/{project_file['name']} {start}")


def finish_exe():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    os.system('cd build/internal && gcc -o main.exe main.o')
    os.rename("build/internal/main.exe", f"build/{project_file['name']}")


def finish_lib():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    os.rename("build/internal/main.o", f"build/lib{project_file['name']}")


def finish():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    if project_file['type'] == "exe":
        finish_exe()
    elif project_file['type'] == "lib":
        finish_lib()
