pub struct Token {
    pub _type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}
impl Token {
    pub fn new(_type: TokenType, start: usize, length: usize, line: usize) -> Self {
        Self {
            _type,
            start,
            length,
            line,
        }
    }
    pub fn get_source(&mut self, source: &Vec<char>) -> String {
        source[self.start..self.start + self.length]
            .iter()
            .collect()
    }
}

pub enum TokenType {
    Keyword(ReservedKeywords),
    Symbol(Symbols),
    Integer(usize),
    String,
    Identifier,
    EOF,
}

pub enum ReservedKeywords {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

pub enum Symbols {
    LeftParam,    // (
    RightParam,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    SemiColon,    // ;
    Equal,        // =
    Plus,         // +
    Minus,        // -
    And,          // &
    Or,           // |
    Bang,         // !
    Tilde,        // ~
    LessThan,     // <
    GreaterThan,  // >
    Comma,        // ,
    Asterisk,     // *
    Slash,        // /
}
