//return
@5
D=A
@LCL
A=M-D // LCL - 5, call_address
D=M // D = *call_address
@SP
A=M
M=D // *SP = *call_address
A=A-1 // rtr_value addr
D=M // D = rtr_value
@ARG
A=M // ARG
M=D // *ARG = rtr_value
D=A // D = ARG
@SP
A=M+1 // SP++
M=D // *SP++ = ARG
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
D=M // D = LCL
@LCL
M=D // set LCL
@SP
A=M+1 // SP++ = ARG
D=M // D = ARG
D=D+A // D = ARG + 1 + SP
@SP
M=D-M // M = ARG + 1 + SP - SP
A=D-M // A = SP + ARG + 1 - ARG - 1
A=M // A = *call_address
0;JMP
