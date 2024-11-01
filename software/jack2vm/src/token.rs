use std::fmt::Display;

#[derive(Debug, Clone)]
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
    pub fn get_source(&self, source: &[char]) -> String {
        match self._type {
            // Ignore the ""
            TokenType::String => source[self.start + 1..self.start + self.length - 1]
                .iter()
                .collect(),

            _ => source[self.start..self.start + self.length]
                .iter()
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Keyword(ReservedKeywords),
    Symbol(Symbols),
    Integer(usize),
    String,
    Identifier,
    EOF,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    Dot,          // .
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_source_not_string() {
        let source = String::new().chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::EOF,
            start: 0,
            length: 0,
            line: 1,
        };
        assert_eq!(token.get_source(&source), String::new());

        let source = "!".chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::Symbol(Symbols::Bang),
            start: 0,
            length: 1,
            line: 1,
        };
        assert_eq!(token.get_source(&source), "!".to_string());

        let source = "class".chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::Keyword(ReservedKeywords::Class),
            start: 0,
            length: 5,
            line: 1,
        };
        assert_eq!(token.get_source(&source), "class".to_string());
    }
    #[test]
    fn get_source_string() {
        let source = "\"funny string\"".chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::String,
            start: 0,
            length: 14,
            line: 1,
        };
        assert_eq!(token.get_source(&source), "funny string".to_string());
    }
}
