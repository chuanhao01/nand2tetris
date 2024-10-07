(LOOP)
    @32
    D=A
    @KBD
    D=D-M
    @BLOCK // If we making the block
    D;JEQ
    // Else we clear the whole screen
    @i
    M=0
    @SCREEN
    D=A
    @cur_screen // = screen
    M=D
(UNDRAW)
    @i
    D=M
    @8192
    D=D-A
    @LOOP
    D;JGE

    @cur_screen
    A=M
    M=0

    @cur_screen
    M=M+1

    @i // i++
    M=M+1
    @UNDRAW
    0;JMP



(BLOCK)
    @i
    M=0
    @SCREEN
    D=A
    @cur_screen // = screen
    M=D

(DRAW_REACT)
    @i
    D=M
    @R0
    D=D-M
    @LOOP
    D;JGE

    @cur_screen
    A=M
    M=-1

    @32
    D=A
    @cur_screen
    M=D+M

    @i // i++
    M=M+1
    @DRAW_REACT
    0;JMP
