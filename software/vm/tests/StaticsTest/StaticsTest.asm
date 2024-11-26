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
//function Class1.set 0
(Class1.set)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
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
//pop static 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Class1.0
M=D
//push argument 1
@ARG
D=M
@1
A=D+A
D=M // D = *(ARG+1)
@SP
A=M
M=D // *SP = D
@SP
M=M+1 // SP++
//pop static 1
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Class1.1
M=D
//push constant 0
@0
D=A // D = 0
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 0
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
//function Class1.get 0
(Class1.get)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
//push static 0
@Class1.0
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//push static 1
@Class1.1
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//sub
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=M-D // *SP-- = *SP-- + D, *SP-- = X - Y
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
//function Class2.set 0
(Class2.set)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
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
//pop static 0
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Class2.0
M=D
//push argument 1
@ARG
D=M
@1
A=D+A
D=M // D = *(ARG+1)
@SP
A=M
M=D // *SP = D
@SP
M=M+1 // SP++
//pop static 1
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@Class2.1
M=D
//push constant 0
@0
D=A // D = 0
@SP
AM=M+1 // *SP+1, SP++
A=A-1
M=D // *SP = 0
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
//function Class2.get 0
(Class2.get)
@0
D=A // D = 0
@SP
M=D+M // SP = SP + 0
//push static 0
@Class2.0
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//push static 1
@Class2.1
D=M
@SP
M=M+1 // SP++
A=M-1
M=D // *SP = D
//sub
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
@SP
A=M-1
M=M-D // *SP-- = *SP-- + D, *SP-- = X - Y
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