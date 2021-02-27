import json
import os

project_type = " "
std = " "


def get_project_type():
    global project_type
    while True:
        project_type = input("Executable or library (exe/lib) ")
        if project_type == "exe":
            break
        elif project_type == "lib":
            break
        else:
            print(f"{project_type} is not 'exe' or 'lib'")


def std_import():
    global std
    while True:
        std = input("Add stdlib to imports (y/n) ")
        if std == "y":
            break
        elif std == "n":
            break
        else:
            print(f"{std} it not 'y' or 'n'")


def main():
    name = input("Project name: ")
    os.mkdir(name)
    os.mkdir(f"{name}/src/")
    os.system(f'touch {name}/project.json')
    project_file = open(f"{name}/project.json", "w")
    project_file.writelines('{\n')
    project_file.writelines(f'  "name": "{name}",\n')
    project_file.writelines('   "version": "1.0.0",\n')
    get_project_type()
    if project_type == "exe":
        os.system(f'touch {name}/src/main.vers')
        project_file.writelines('   "type": "exe",\n')
    elif project_type == "lib":
        os.system(f'touch {name}/src/lib.vers')
        project_file.writelines('   "type": "lib",\n')
    std_import()
    if std == "y":
        project_file.writelines('   "std": true\n')
    elif std == "n":
        project_file.writelines('   "std": false\n')
    project_file.writelines('}\n')


if __name__ == '__main__':
    main()
