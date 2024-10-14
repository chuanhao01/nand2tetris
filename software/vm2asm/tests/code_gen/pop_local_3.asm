//pop local 3
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@LCL
D=D+M // *SP + LCL
@3
D=D+A // *SP + (LCL+i)
@SP
A=M // *SP
A=M // A = *SP
A=D-A // A = *SP + (LCL+i) - *SP, A = (LCL+i)
M=D-A // *(LCL+i) = *SP + (LCL+i) - (LCL+i)
