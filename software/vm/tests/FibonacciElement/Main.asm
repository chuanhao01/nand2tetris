//function Main.fibonacci 0
(Main.fibonacci)
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
//push constant 2
@2
D=A // D = 2
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 2
//lt
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
A=A-1 // SP--
D=M-D // D = *SP-- - D, D=x-y
M=-1 // *SP-- = true
@Main.lt.0
D;JLT
@SP
A=M-1
M=0 // *SP-- = false
(Main.lt.0)
//if-goto N_LT_2
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Main.fibonacci$N_LT_2
D;JNE
//goto N_GE_2
@Main.fibonacci$N_GE_2
0;JMP
//label N_LT_2
(Main.fibonacci$N_LT_2)
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
//label N_GE_2
(Main.fibonacci$N_GE_2)
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
//push constant 2
@2
D=A // D = 2
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 2
//sub
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=M-D // *SP-- = *SP-- + D, *SP-- = X - Y
//call Main.fibonacci 1, Main
@Main.Main.fibonacci.return.0 // push @Main.fibonacci.return.0
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Main.fibonacci.return.0
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
@SP
D=M
@LCL
M=D // set LCL
@6
D=D-A // D = SP - 6
@ARG
M=D // set ARG
@Main.fibonacci
0;JMP
(Main.Main.fibonacci.return.0)
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
//push constant 1
@1
D=A // D = 1
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 1
//sub
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=M-D // *SP-- = *SP-- + D, *SP-- = X - Y
//call Main.fibonacci 1, Main
@Main.Main.fibonacci.return.1 // push @Main.fibonacci.return.1
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Main.fibonacci.return.1
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
@SP
D=M
@LCL
M=D // set LCL
@6
D=D-A // D = SP - 6
@ARG
M=D // set ARG
@Main.fibonacci
0;JMP
(Main.Main.fibonacci.return.1)
//add
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=D+M // *SP-- = *SP-- + D, *SP-- = X + Y
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