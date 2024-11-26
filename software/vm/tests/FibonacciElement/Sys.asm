//function Sys.init 0
(Sys.init)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
//push constant 4
@4
D=A // D = 4
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 4
//call Main.fibonacci 1, Sys
@Sys.Main.fibonacci.return.0 // push @Main.fibonacci.return.0
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
(Sys.Main.fibonacci.return.0)
//label END
(Sys.init$END)
//goto END
@Sys.init$END
0;JMP