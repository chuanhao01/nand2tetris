# Wow

## Constants

```
RAM[0]: SP - Stack Pointer
RAM[1]: LCL - Local
RAM[2]: ARG - argument
RAM[3]: THIS - this
RAM[4]: THAT - that
RAM[5-12]: temp
RAM[13-15]: General purpose registers
```

## Memory Segments

```
local - LCL
argument - ARG
this
that

pointer - pointer 0 is THIS
pointer 1 is THAT


temp - RAM 5 - 12

constant - virtual

static - FileName.i where i is static i
```

### Behaviour

pointer
```
push pointer 0/1
```

Should do
```
*SP = THIS/THAT
SP++
```

Static
```
push static i
```

```

```
