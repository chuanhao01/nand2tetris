# Wow

Everything I need to implement for part 1

Also everything can be found in `software/vm2asm`

```
add
sub
neg
eq
gt
lt
and
or
not

pop segment i
push segment i

label label
goto label
if-goto label

function functionName nvars
call functionname nargs
return
```

Compiler will be based on first kw

## Commands

The VM represents true and false as -1 (minus one, 0xFFFF ) and 0 (zero, 0x0000),
respectively.

Because !false (!0) = -1
And !true (!-1) = 0

For commands using 2 involving 2 numbers
It is x then y

So
X
Y
SP

Commands

```
add

@SP 
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1 
M=M+D // *SP-- = *SP-- + D
```

```
sub (x-y)

@SP 
AM=M-1 // SP = SP--
D=M // D = *SP, Y
@SP
A=M-1 
M=M-D // *SP-- = *SP-- - D, *SP-- = X - Y
```

```
neg

@SP
A=M-1 
M=-M // *SP-- = -*SP--
```

```
eq

@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
D=M-D // D = *SP-- - D, D=x-y, x==y, D==0
M=0 // *SP-- = false
@FileName.eq.i
D;JEQ
@SP // If not equal, M=true, for M=!true later
A=M-1
M=-1
(FileName.eq.i) // If equal, M=!false, M=true
@SP 
A=M-1
M=!M 
```

@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
D=M-D // D = *SP-- - D, D=x-y, x==y, D==0
M=-1 // *SP-- = true
@FileName.eq.i
D;JEQ
@SP
A=M-1
M=0
(FileName.eq.i)


```
gt(x>y)

@SP
AM=M-1 // SP = SP--
D=M // D = *SP, Y
@SP
A=M-1
D=M-D // D = *SP-- - D, D = x-y
M=0 // *SP-- = false
@Filename.gt.i
D;JGT
@SP // If not condition, M=true for !M to be false later
A=M-1
M=-1
(Filename.gt.i)
@SP
A=M-1
M=!M // Flips M
```

```
lt

@SP
AM=M-1 // SP = SP--
D=M // D = *SP, Y
@SP
A=M-1
D=M-D // D = *SP-- - D, D = x-y
M=0 // *SP-- = false
@Filename.lt.i
D;JLT
@SP // If not condition, M=true for !M to be false later
A=M-1
M=-1
(Filename.lt.i)
@SP
A=M-1
M=!M // Flips M
```

```
and

@SP
AM=M-1 // SP = SP--
D=M
@SP
A=M-1
M=D&M
```

```
or


@SP
AM=M-1 // SP = SP--
D=M
@SP
A=M-1
M=D|M
```

```
not

@SP
A=M-1
M=!M
```

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

*SP = *FileName.i
SP++
```

```
pop static i

SP--
*FileName.i = *SP
```

Temp
```
push temp i

*SP = *RAM[5+i]
SP++
```

### Push

@LCL
D=M
@i
A=D+A
D=M // D = *(LCL+i)
@SP
A=M
M=D // *SP = D
@SP
M=M+1 // SP++

### Pop

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

### Pointer

0/1
THIS/THAT

push

@THIS
D=M
@SP
M=M+1
A=M-1
M=D

pop

@SP
M=M-1
A=M
D=M
@THIS
M=D

### Temp

Push

@5+i
D=M
@SP
M=M+1
A=M-1
M=D

Pop

@SP
M=M-1
A=M
D=M
@5+1
M=D

### Static

Push

@Filename.i
D=M
@SP
M=M+1
A=M-1
M=D

Pop

@SP
M=M-1
A=M
D=M
@Filename.i
M=D
