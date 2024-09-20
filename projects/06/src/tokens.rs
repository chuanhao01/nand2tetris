#[derive(PartialEq, Debug)]
pub enum Token {
    NormalToken {
        _type: TokenType,
        start: usize,
        length: usize,
        line: usize,
    },
    ErrorToken {
        line: usize,
        msg: String,
    },
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Single Characters
    LeftParam,
    RightParam,
    SemiColon,
    At,
    Equal,
    NewLine,
    // -- comp
    Plus,
    Minus,
    And,
    Or,
    Bang,
    // Literals
    Number,
    Label,
    // Keywords
    NULL,
    // -- dest
    A,
    D,
    M,
    AD,
    AM,
    MD,
    AMD,
    // -- Jump
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
    // -- Pre defined symbols
    SP,
    LCL,
    ARG,
    THIS,
    THAT,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    KBD,
    SCREEN,

    // Final
    EOF,
}
