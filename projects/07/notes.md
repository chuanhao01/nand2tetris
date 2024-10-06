VM Code to Assembly Translator

push / pop segment i

## Using pointers, Pushing Constants

push constant i

Uses SP pointer

R0 - SP
...
R256 - 1

pseudo code
```
*SP = constant // Sets SP
SP++ // Moves SP by 1 to the next register
```

Looks like this:
```
@10
D=A // D = 10
@SP
A=M
M=D // RAM[SP] = D, *SP = D
@SP // SP++
M=M+1
```

## Pushing and Popping from other segments of memory

Example: `pop local 2`

1. Find address of `local 2`
2. Decrement SP
3. Put SP into `local 2`

```
addr = LCL + 2
SP--
*addr = *SP
```

Looks like:
POG
```
@2
D=A
@LCL
D=D+M
@SP
A=M
M=D
@SP
M=M-1
A=M
D=M
@SP
A=M+1
A=M
M=D
```
