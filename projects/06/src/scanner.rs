use crate::{Token, TokenType};

pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn scan_token(&mut self, source: &Vec<char>) -> Token {
        self.skip_whitespace(source);
        if self.is_at_end(source) {
            return self.make_token(TokenType::EOF);
        }

        // Update start before parsing next valid token
        self.start = self.current;
        let c = self.advance(source).unwrap(); // Should always be a valid advance
                                               // Found an identifier
        if self.is_alpha(c) {
            return self.identifier(source);
        }
        if self.is_numeric(c) {
            return self.number(source);
        }
        match c {
            '(' => self.make_token(TokenType::LeftParam),
            ')' => self.make_token(TokenType::RightParam),
            ';' => self.make_token(TokenType::SemiColon),
            '@' => self.make_token(TokenType::At),
            '=' => self.make_token(TokenType::Equal),
            '\n' => {
                self.line += 1;
                self.make_token(TokenType::NewLine)
            }
            _ => self.make_error_token(String::from("Unexpected Character")),
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_' || c == '.' || c == '$' || c == ':'
    }
    fn is_numeric(&self, c: char) -> bool {
        c.is_numeric()
    }

    fn is_at_end(&self, source: &Vec<char>) -> bool {
        self.current >= source.len()
    }
    // Moves pointer forward, returns the current char
    fn advance(&mut self, source: &Vec<char>) -> Option<char> {
        self.current += 1;
        if self.current > source.len() {
            None
        } else {
            Some(source[self.current - 1])
        }
    }
    fn peek(&self, source: &Vec<char>) -> char {
        // Will never fail, as calls to peek should be on a valid character
        source[self.current]
    }
    fn peek_n(&self, source: &Vec<char>, n: usize) -> Option<char> {
        if self.current + n >= source.len() {
            None
        } else {
            Some(source[self.current + n])
        }
    }
    fn peek_next(&self, source: &Vec<char>) -> Option<char> {
        self.peek_n(source, 1)
    }

    fn skip_whitespace(&mut self, source: &Vec<char>) {
        loop {
            if self.is_at_end(source) {
                return;
            }
            let c = self.peek(source);
            match c {
                // Whitespace and tabs
                ' ' | '\t' | '\r' => {
                    self.advance(source);
                }
                // Comments
                '/' => {
                    // Manual peek forward
                    if let Some(next_c) = self.peek_next(source) {
                        if next_c == '/' {
                            self.current += 2; // Skip the commment prefix, //

                            // Skip until we reach the end of the line or EOF
                            while !self.is_at_end(source) && self.peek(source) != '\n' {
                                self.advance(source);
                            }
                        } else {
                            // Not a comment, there are more valid token later on
                            // Pass back control
                            return;
                        }
                    } else {
                        // There is no peek, pass back control
                        return;
                    }
                }
                // If we hit something that is not whitespace, return back control
                _ => return,
            }
        }
    }

    fn identifier(&mut self, source: &Vec<char>) -> Token {
        // While not at the end, consume the whole identifier
        while !self.is_at_end(source)
            && (self.is_alpha(self.peek(source)) || self.is_numeric(self.peek(source)))
        {
            self.advance(source);
        }

        self.make_token(self.identifier_type(source))
    }
    /// Very long trie, done by hand
    fn identifier_type(&self, source: &Vec<char>) -> TokenType {
        match source[self.start] {
            'n' => self.check_reserved_keyword(source, 1, String::from("ull"), TokenType::NULL),
            'D' => self.check_reserved_keyword(source, 1, String::default(), TokenType::D),
            'L' => self.check_reserved_keyword(source, 1, String::from("CL"), TokenType::LCL),
            'K' => self.check_reserved_keyword(source, 1, String::from("BD"), TokenType::KBD),
            'A' => {
                if self.current - self.start > 1 {
                    match source[self.start + 1] {
                        'D' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::AD)
                        }
                        'R' => self.check_reserved_keyword(
                            source,
                            2,
                            String::from("G"),
                            TokenType::ARG,
                        ),
                        'M' => {
                            if self.current - self.start > 2 {
                                self.check_reserved_keyword(
                                    source,
                                    2,
                                    String::from("D"),
                                    TokenType::AMD,
                                )
                            } else {
                                // AM
                                self.check_reserved_keyword(
                                    source,
                                    2,
                                    String::default(),
                                    TokenType::AM,
                                )
                            }
                        }
                        _ => TokenType::Label,
                    }
                } else {
                    // A
                    self.check_reserved_keyword(source, 1, String::default(), TokenType::A)
                }
            }
            'M' => {
                if self.current - self.start > 1 {
                    self.check_reserved_keyword(source, 1, String::from("D"), TokenType::MD)
                } else {
                    self.check_reserved_keyword(source, 1, String::default(), TokenType::M)
                }
            }
            'J' => {
                if self.current - self.start > 1 {
                    match source[self.start + 1] {
                        'E' => self.check_reserved_keyword(
                            source,
                            2,
                            String::from("Q"),
                            TokenType::JEQ,
                        ),
                        'N' => self.check_reserved_keyword(
                            source,
                            2,
                            String::from("E"),
                            TokenType::JNE,
                        ),
                        'M' => self.check_reserved_keyword(
                            source,
                            2,
                            String::from("P"),
                            TokenType::JMP,
                        ),
                        'G' => {
                            if self.current - self.start > 2 {
                                match source[self.start + 2] {
                                    'T' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::JGT,
                                    ),
                                    'E' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::JGE,
                                    ),
                                    _ => TokenType::Label,
                                }
                            } else {
                                TokenType::Label
                            }
                        }
                        'L' => {
                            if self.current - self.start > 2 {
                                match source[self.start + 2] {
                                    'T' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::JLT,
                                    ),
                                    'E' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::JLE,
                                    ),
                                    _ => TokenType::Label,
                                }
                            } else {
                                TokenType::Label
                            }
                        }
                        _ => TokenType::Label,
                    }
                } else {
                    TokenType::Label
                }
            }
            'S' => {
                if self.current - self.start > 1 {
                    match source[self.start + 1] {
                        'P' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::SP)
                        }
                        'C' => self.check_reserved_keyword(
                            source,
                            2,
                            String::from("REEN"),
                            TokenType::SCREEN,
                        ),
                        _ => TokenType::Label,
                    }
                } else {
                    TokenType::Label
                }
            }
            'T' => {
                if self.current - self.start > 2 {
                    match (source[self.start + 1], source[self.start + 2]) {
                        ('H', 'A') => self.check_reserved_keyword(
                            source,
                            3,
                            String::from("T"),
                            TokenType::THAT,
                        ),
                        ('H', 'I') => self.check_reserved_keyword(
                            source,
                            3,
                            String::from("S"),
                            TokenType::THIS,
                        ),
                        _ => TokenType::Label,
                    }
                } else {
                    TokenType::Label
                }
            }
            'R' => {
                if self.current - self.start > 1 {
                    match source[self.start + 1] {
                        '0' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R0)
                        }
                        '2' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R2)
                        }
                        '3' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R3)
                        }
                        '4' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R4)
                        }
                        '5' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R5)
                        }
                        '6' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R6)
                        }
                        '7' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R7)
                        }
                        '8' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R8)
                        }
                        '9' => {
                            self.check_reserved_keyword(source, 2, String::default(), TokenType::R9)
                        }
                        '1' => {
                            if self.current - self.start > 2 {
                                match source[self.start + 2] {
                                    '0' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::R10,
                                    ),
                                    '1' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::R11,
                                    ),
                                    '2' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::R12,
                                    ),
                                    '3' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::R13,
                                    ),
                                    '4' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::R14,
                                    ),
                                    '5' => self.check_reserved_keyword(
                                        source,
                                        3,
                                        String::default(),
                                        TokenType::R15,
                                    ),
                                    _ => TokenType::Label,
                                }
                            } else {
                                TokenType::Label
                            }
                        }
                        _ => TokenType::Label,
                    }
                } else {
                    TokenType::Label
                }
            }
            _ => TokenType::Label,
        }
    }
    fn check_reserved_keyword(
        &self,
        source: &Vec<char>,
        start: usize,
        rest: String,
        token_type: TokenType,
    ) -> TokenType {
        let length = rest.len();
        // Length has to be the same
        // The chars of the rest has to match up
        if self.current - self.start == start + length
            && source[(self.start + start)..(self.start + start + length)]
                == rest.chars().collect::<Vec<char>>()
        {
            return token_type;
        }
        TokenType::Label
    }

    fn number(&mut self, source: &Vec<char>) -> Token {
        while !self.is_at_end(source) && self.is_numeric(self.peek(source)) {
            self.advance(source);
        }

        self.make_token(TokenType::Number)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::NormalToken {
            _type: token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
    fn make_error_token(&self, msg: String) -> Token {
        Token::ErrorToken {
            line: self.line,
            msg,
        }
    }
}
impl Default for Scanner {
    fn default() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Figure out how to make a test call with input params
    // To declarative create tests
    #[test]
    fn test_skip_whitespace_eof() {
        // skip_whitespace should skip to the end of file correctly
        let seq = vec!["      ", "//", "// ", "   // something interesting"];
        for s in seq {
            let mut scanner = Scanner::new();
            let source = s.chars().collect::<Vec<_>>();
            scanner.skip_whitespace(&source);
            assert!(scanner.is_at_end(&source));
        }
    }

    #[test]
    fn test_skip_whitespace_next() {
        let seq = vec![
            ("\n", '\n'),
            ("   so", 's'),
            ("//\n", '\n'),
            ("//nothing here \n", '\n'),
            ("     \n", '\n'),
        ];
        for (s, expected) in seq {
            let mut scanner = Scanner::new();
            let source = s.chars().collect::<Vec<_>>();
            scanner.skip_whitespace(&source);
            let c = scanner.advance(&source).unwrap(); // If there no next char, should fail
            assert_eq!(c, expected);
        }
    }

    #[test]
    fn test_identifier() {
        let seq = Vec::from([
            (
                "R10",
                Token::NormalToken {
                    _type: TokenType::R10,
                    start: 0,
                    length: 3,
                    line: 1,
                },
            ),
            (
                "SCREEN",
                Token::NormalToken {
                    _type: TokenType::SCREEN,
                    start: 0,
                    length: 6,
                    line: 1,
                },
            ),
        ]);
        for (s, expected) in seq {
            let mut scanner = Scanner::new();
            let source = s.chars().collect::<Vec<_>>();
            let t = scanner.scan_token(&source);
            assert_eq!(t, expected);
        }
    }

    #[test]
    fn test_number() {
        let s = "@10E";
        let mut scanner = Scanner::new();
        let source = s.chars().collect::<Vec<_>>();
        let t = scanner.scan_token(&source);
        assert_eq!(
            t,
            Token::NormalToken {
                _type: TokenType::At,
                start: 0,
                length: 1,
                line: 1
            }
        );
        let t = scanner.scan_token(&source);
        assert_eq!(
            t,
            Token::NormalToken {
                _type: TokenType::Number,
                start: 1,
                length: 2,
                line: 1
            }
        );
        let t = scanner.scan_token(&source);
        assert_eq!(
            t,
            Token::NormalToken {
                _type: TokenType::Label,
                start: 3,
                length: 1,
                line: 1
            }
        );
    }
}
