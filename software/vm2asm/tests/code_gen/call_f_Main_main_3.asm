//call Main.main 3, f
@f.Main.main.return.0 // push @Main.main.return.0
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
@8
D=D-A // D = SP - 8
@ARG
M=D // set ARG
@Main.main
0;JMP
(f.Main.main.return.0)
