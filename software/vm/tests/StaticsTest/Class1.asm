//function Class1.set 0
(Class1.set)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
//push argument 0
@ARG
D=M
@0
A=D+A
D=M // D = *(ARG+0)
@SP
A=M
M=D // *SP = D
@SP
M=M+1 // SP++
//pop static 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Class1.0
M=D
//push argument 1
@ARG
D=M
@1
A=D+A
D=M // D = *(ARG+1)
@SP
A=M
M=D // *SP = D
@SP
M=M+1 // SP++
//pop static 1
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Class1.1
M=D
//push constant 0
@0
D=A // D = 0
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 0
//return
@SP
A=M-1 // rtr_value addr
D=M
@ARG
A=M // ARG
M=D // *ARG = rtr_value
D=A
@SP
M=D+1 // SP = ARG + 1
@LCL
AM=M-1 // LCL - 1
D=M // D = THAT
@THAT
M=D // set THAT
@LCL
AM=M-1 // LCL - 2
D=M // D = THIS
@THIS
M=D // set THIS
@LCL
AM=M-1 // LCL - 3
D=M // D = ARG
@ARG
M=D // set ARG
@LCL // LCL - 3
AM=M-1 // LCL - 4
D=A // D = LCL - 4
D=D+M // D = LCL - 4 + old_LCL value
@LCL
M=D-M // M = old_LCL value
A=D-M // A = LCL - 4
A=A-1 // LCL - 5, call_address
A=M // A = call_address value
0;JMP
//function Class1.get 0
(Class1.get)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
//push static 0
@Class1.0
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//push static 1
@Class1.1
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//sub
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=M-D // *SP-- = *SP-- + D, *SP-- = X - Y
//return
@SP
A=M-1 // rtr_value addr
D=M
@ARG
A=M // ARG
M=D // *ARG = rtr_value
D=A
@SP
M=D+1 // SP = ARG + 1
@LCL
AM=M-1 // LCL - 1
D=M // D = THAT
@THAT
M=D // set THAT
@LCL
AM=M-1 // LCL - 2
D=M // D = THIS
@THIS
M=D // set THIS
@LCL
AM=M-1 // LCL - 3
D=M // D = ARG
@ARG
M=D // set ARG
@LCL // LCL - 3
AM=M-1 // LCL - 4
D=A // D = LCL - 4
D=D+M // D = LCL - 4 + old_LCL value
@LCL
M=D-M // M = old_LCL value
A=D-M // A = LCL - 4
A=A-1 // LCL - 5, call_address
A=M // A = call_address value
0;JMP