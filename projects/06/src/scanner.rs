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
        match c {
            '(' => self.make_token(TokenType::LeftParam),
            ')' => self.make_token(TokenType::RightParam),
            ';' => self.make_token(TokenType::SemiColon),
            '@' => self.make_token(TokenType::At),
            '=' => self.make_token(TokenType::Equal),
            '\n' => self.make_token(TokenType::NewLine),
            _ => self.make_error_token(String::from("Unexpected Character")),
        }
    }

    fn is_at_end(&self, source: &Vec<char>) -> bool {
        self.current >= source.len()
    }
    // Moves pointer forward, returns the current char
    fn advance(&mut self, source: &Vec<char>) -> Option<char> {
        self.current += 1;
        if self.current - 1 >= source.len() {
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

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::NormalToken {
            _type: token_type,
            start: self.start,
            length: self.current,
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
}
