### Inline Assembly

To write inline assembly in Vers, you need to use the ``asm`` block.

``asm`` and ``end_asm``
```vers
asm at hlt end_asm
```
In real assembly this is what it would be:
```asm
    hlt
```

So what is the ``at`` for? Inline assembly in Vers gets written to an assembly file word by word so to make a tab or 
space you would use ``at`` assembly tab or ``al`` assembly line
