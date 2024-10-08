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

The overflow problem when doing *SP + (LCL+i) is fine since we are using 2's compliment
So when we do *SP + (LCL+i) - *SP we get (LCL+i) and vice versa

@SP
AM=M-1 // SP-- & *SP
D=M // D = *SP
@LCL
D=D+M // D = *SP + LCL
@i
D=D+A // D = *SP + (LCL+i)
@SP
A=M // *SP
A=M // A = *SP
A=D-A // A = *SP + (LCL+i) - *SP, *(LCL+i)
M=D-A // *(LCL+i) = *SP + (LCL+i) - (LCL+i)
