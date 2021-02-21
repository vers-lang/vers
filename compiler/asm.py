import json
import os


def finish_exe():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    os.system('cd build/internal && gcc -o main.exe main.o')
    os.rename("build/internal/main.exe", f"build/{project_file['name']}")


def finish_lib():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    os.rename("build/internal/main.o", f"build/{project_file['name']}")


def compile_asm():
    read_project_file = open("project.json").read()
    project_file = json.loads(read_project_file)
    os.system('cd build/internal && gcc -c main.S')
    if project_file['type'] == "exe":
        finish_exe()
