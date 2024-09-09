@i
M=0
(LOOP)
    @R1
    D=M
    @i
    D=D-M
    @END
    D;JEQ

    @R0
    D=M

    @R2
    M=D+M

    @i // i++
    M=M+1
    @LOOP
    0;JMP

(END)
    @END
    0;JMP
