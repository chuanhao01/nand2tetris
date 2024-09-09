@R0
D=M
@R1
D=D-M
@RIGHT
D;JLE

@R0
D=M

@SET
0;JMP

(RIGHT)
    @R1
    D=M

(SET)
    @R2
    M=D


(END)
    @END
    0;JMP
