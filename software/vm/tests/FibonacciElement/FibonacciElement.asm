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
//function Main.fibonacci 0
(Main.fibonacci)
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