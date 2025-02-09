@256
D=A
@SP
M=D
//call Sys.init 0, bootstrap
@bootstrap.Sys.init.return.0 // push @Sys.init.return.0
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Sys.init.return.0
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
@5
D=D-A // D = SP - 5
@ARG
M=D // set ARG
@Sys.init
0;JMP
(bootstrap.Sys.init.return.0)
//function Sys.init 0
(Sys.init)
//push constant 0
@0
D=A // D = 0
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 0
//call Main.main 1, Sys
@Sys.Main.main.return.0 // push @Main.main.return.0
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Main.main.return.0
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
@Main.main
0;JMP
(Sys.Main.main.return.0)
//pop temp 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@5
M=D
//push constant 0
@0
D=A // D = 0
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 0
//label END
(Sys.init$END)
//goto END
@Sys.init$END
0;JMP
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
//function Main.main 0
(Main.main)
//push constant 1
@1
D=A // D = 1
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 1
//pop static 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Main.0
M=D
//push constant 1
@1
D=A // D = 1
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 1
//pop static 1
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Main.1
M=D
//push static 0
@Main.0
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//push static 1
@Main.1
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//add
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=D+M // *SP-- = *SP-- + D, *SP-- = X + Y
//pop static 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Main.0
M=D
//push constant 0
@0
D=A // D = 0
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 0
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