//add
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=M+D // *SP-- = *SP-- + D, *SP-- = X + Y
