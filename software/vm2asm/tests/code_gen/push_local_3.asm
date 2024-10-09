//push local 3
@LCL
D=M
@3
A=D+A
D=M // D = *(LCL+3)
@SP
A=M
M=D // *SP = D
@SP
M=M+1 // SP++
