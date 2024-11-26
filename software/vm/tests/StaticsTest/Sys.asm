//function Sys.init 0
(Sys.init)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
//push constant 6
@6
D=A // D = 6
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 6
//push constant 8
@8
D=A // D = 8
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 8
//call Class1.set 2, Sys
@Sys.Class1.set.return.0 // push @Class1.set.return.0
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Class1.set.return.0
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
@7
D=D-A // D = SP - 7
@ARG
M=D // set ARG
@Class1.set
0;JMP
(Sys.Class1.set.return.0)
//pop temp 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@5
M=D
//push constant 23
@23
D=A // D = 23
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 23
//push constant 15
@15
D=A // D = 15
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 15
//call Class2.set 2, Sys
@Sys.Class2.set.return.1 // push @Class2.set.return.1
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Class2.set.return.1
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
@7
D=D-A // D = SP - 7
@ARG
M=D // set ARG
@Class2.set
0;JMP
(Sys.Class2.set.return.1)
//pop temp 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@5
M=D
//call Class1.get 0, Sys
@Sys.Class1.get.return.2 // push @Class1.get.return.2
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Class1.get.return.2
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
@Class1.get
0;JMP
(Sys.Class1.get.return.2)
//call Class2.get 0, Sys
@Sys.Class2.get.return.3 // push @Class2.get.return.3
D=A
@SP
AM=M+1 // SP++
A=A-1 // SP
M=D // Class2.get.return.3
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
@Class2.get
0;JMP
(Sys.Class2.get.return.3)
//label END
(Sys.init$END)
//goto END
@Sys.init$END
0;JMP