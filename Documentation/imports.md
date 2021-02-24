### Imports

Vers doesn't have any builtin functions, so you have to import a function or functions from a library. You can install a
library by compiling the installed or written Vers project and putting the compiled file ``build/liblibname`` to 
``/lib/vers/lib`` and the rest of the directory in ``/lib/vers/``. The Vers stdlib is automatically imported if you have
``std`` selected as ``true`` in ``project.json``.

Stdlib:
```json
"std": true,
```

No stdlib:
```json
"std": false,
```

Importing:
```vers
import library;
use library_function;
use other_library_function;
```
