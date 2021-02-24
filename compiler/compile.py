import os


def compile_internal():
    os.system('cd build/internal && gcc -c main.S')


def compile_external():
    os.system('cd build/external && gcc -c *.c')
