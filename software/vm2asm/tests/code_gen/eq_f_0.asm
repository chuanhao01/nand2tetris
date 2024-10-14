//eq
@SP
AM=M-1 // SP = SP--
D=M // D = *SP
A=A-1 // SP--
D=M-D // D = *SP-- - D, D=x-y
M=-1 // *SP-- = true
@f.eq.0
D;JEQ
@SP
A=M-1
M=0 // *SP-- = false
(f.eq.0)
