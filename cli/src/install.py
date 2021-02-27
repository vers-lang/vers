import os


def main():
    website = input("Install website name (github.com): ")
    lib = input("Library name: ")
    author = input("Author of library: ")
    os.system(f'git clone https://{website}/{author}/{lib}')
    os.system(f'mv lib/ /home/$USER/.vers/')
    os.system(f'cd /home/$USER/.vers/{lib}/ && vers build')
