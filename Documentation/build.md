### Build Vers-lang

Setup environment:
```commandline
python3 -m venv venv
source venv/bin/activate
```

Get the requirements for compiling Vers:
```commandline
pip install -r requirements.txt
```

Build Vers lang with pyinstaller:
```commandline
pyinstaller --onefile vers.py
```
Then move ``dist/vers`` to ``/bin/vers`` and compile the Vers stdlib
```commandline
cd std/ && vers build
mv build/libstd /lib/vers/lib/
```

Or

```commandline
sudo sh build.sh
sudo sh lib.sh
```
Running this moves the ``vers`` file to the ``/bin/`` directory.

---
Read:

``hello_world.md``
``commands.md``
