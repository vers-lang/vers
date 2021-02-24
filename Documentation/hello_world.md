### Hello World! (concept)

Hello World example:
```vers
import std;
use print;
decfun main

fun main() {
    print("Hello world!")
}
```

If you don't have an x86/x86_64 machine use the ``asm`` block to make an syscall.

ARM32 example:

```vers
decfun main
/* std: false */

fun main() {
    asm
    mov r0, #1
    ldr r1, =msg
    ldr r2, =len
    mov r7, #4
    swi #0
    end_asm
    asm
    msg:
        .ascii "Hello world!\n"
    len = . - msg
    end_asm
}
```

---
Read:

``commands.md``
