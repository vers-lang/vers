import os


def install_lib(author, lib, libversion):
    print(f"Installig {lib} {libversion}..")
    os.system(f'cd /home/$USER/verslib/ && git clone https://github.com/{author}/{lib}.git')
    print(f"Updating {lib}...")
    os.system(f'cd /home/$USER/verslib/{lib}/ && git checkout tags/{libversion} -b main')
    print(f"Installed {lib} {libversion} by {author}")
