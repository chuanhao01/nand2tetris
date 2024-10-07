@f1
M=1
@f2
M=1
@2
D=A
@i
M=D
(LOOP)
    @i
    D=M
    @R0
    D=D-M
    @OUT
    D;JGE

    @f1
    D=M
    @f2
    D=D+M
    @f3 // f3 = f1 + f2
    M=D

    @f2 // f1 = f2
    D=M
    @f1
    M=D

    @f3 // f2 = f3
    D=M
    @f2
    M=D

    @i
    M=M+1
    @LOOP
    0;JMP


(OUT)
    @f2
    D=M
    @R1
    M=D
(END)
    @END
    0;JMP
