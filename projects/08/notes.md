# Note
The VM represents true and false as -1 (minus one, 0xFFFF ) and 0 (zero, 0x0000), respectively.

Treat every other value that is not -1 or 0 as true
Because thats the behaviour for `if-goto label` WHY?

Also for labels

Labels within functions should already contain file name, so just take the label as is
so `function function_name`
should produce `(function_name)`

label in a function should generate
`(function_name$label_name)`

# From the book

8.2.1 Program Flow Commands
The VM language features three program flow commands:

label `label`
This command labels the current location in the function’s code.
Only labeled locations can be jumped to from other parts of the program. The scope
of the label is the function in which it is defined.
The label is an arbitrary string composed of any sequence of letters, digits, underscore (_), dot (.), and colon (:) that does not begin with a digit.

m goto label This command effects an unconditional goto operation, causing execution to continue from the location marked by the label. The jump destination must
be located in the same function.
m if-goto label This command effects a conditional goto operation. The stack’s
topmost value is popped; if the value is not zero, execution continues from the location marked by the label; otherwise, execution continues from the next command in
the program. The jump destination must be located in the same function.

## Fucntions

call nargs
(nargs here is how many args it takes)

You have to push the arguments onto the stack

- push the return address
- save states
  - push LCL
  - push ARG
  - push THIS
  - push THAT
- Set ARG = SP - 5 - nargs (new ARG value)
- Set LCL = SP
- goto function
- (return address)

```
@file_name.function_name.return.i // push file_name.function_name.return.i
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // file_name.function_name.return.i
@LCL // push LCL
D=M
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // LCL
@ARG // push ARG
D=M
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // ARG
@THIS // push THIS
D=M
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // THIS
@THAT // push THAT
D=M
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // THAT
@SP // set ARG = SP - 5 - nargs and LCL = SP
D=M
@LCL
M=D // set LCL
@5+nargs
D=D-A // D = SP - 5 - nargs
@ARG
M=D // set ARG
@file_name.function_name
0;JMP
(file_name.function_name.return.i)
```

function nargs
(nargs here is how many local vars)

- generate label
- push 0 * n times
  - Build local segment

```
(file_name.function_name)
@nargs // Move SP to allocate local vars
D=A
@SP
M=M+D // SP = SP + nargs
```

return 

- endframe = LCL
- return address = LCL - 5
- *ARG = pop() (since the return value last pushed value, we move that value into ARG, removing the ARG values and subsituting the return)
- SP = ARG + 1
- THIS
- THAT
- ARG
- LCL
- goto return address

```
@SP
A=M-1 // return value addr
D=M
@ARG
A=M // ARG
M=D // *ARG = pop() - return value

D=A
@SP
M=D+1 // SP = ARG + 1

@LCL
AM=M-1 // AM = LCL - 1
D=M // D = THAT
@THAT
M=D // set THAT
@LCL
AM=M-1 // AM = LCL - 2
D=M // D = THIS
@THIS
M=D
@LCL
AM=M-1 // AM = LCL - 3
D=M // D = ARG
@ARG
M=D

@LCL // LCL - 3 (Some cursed shit)
AM=M-1 // LCL - 4
D=A+M // LCL - 4 + old LCL (TODO, need to split to 2 commands)
@LCL
M=D-M // M = old LCL, (LCL - 4) - (LCL - 4) + old LCL
A=D-M // A = LCL - 4, old LCL - old LCL + LCL - 4
A=A-1 // LCL - 5, return address
A=M // A = *return address
0;JMP
```
