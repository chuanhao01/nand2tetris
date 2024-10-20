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
