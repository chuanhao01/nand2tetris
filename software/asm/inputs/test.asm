//push constant 3
@3
D=A // D = 3
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 3
//push constant 4
@4
D=A // D = 4
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 4
//add
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=D+M // *SP-- = *SP-- + D, *SP-- = X + Y