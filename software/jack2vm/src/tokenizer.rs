use crate::{Token, TokenType};

struct Tokenizer {
    start: usize,
    current: usize,
    line: usize,
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn scan_token(&mut self, source: &Vec<char>) -> Result<Token, String> {
        if let Some(err) = self.skip_whitespace_and_comments(source) {
            return Err(err);
        };
        if self.is_at_end(source) {
            return Ok(Token::new(
                TokenType::EOF,
                self.start,
                self.current - self.start,
                self.line,
            ));
        }
        self.start = self.current;

        Ok(Token::new(
            TokenType::EOF,
            self.start,
            self.current - self.start,
            self.line,
        ))
    }
    fn is_at_end(&mut self, source: &Vec<char>) -> bool {
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
    fn skip_whitespace_and_comments(&mut self, source: &Vec<char>) -> Option<String> {
        loop {
            if self.is_at_end(source) {
                return None;
            }
            let c = self.peek(source);
            if c == ' ' || c == '\t' || c == '\r' {
                // Skip whitespace
                self.advance(source);
            } else if c == '\n' {
                // new line
                self.line += 1;
                self.advance(source);
            } else if c == '/' {
                // Could be start of comment
                let nc = self.peek_next(source);
                match nc {
                    Some(nc) => {
                        match nc {
                            '/' => {
                                // // comment until end of line
                                self.current += 2; // Skip the //
                                while !self.is_at_end(source) && self.peek(source) != '\n' {
                                    self.advance(source);
                                }
                            }
                            '*' => {
                                self.current += 2; // Skip /*
                                loop {
                                    while !self.is_at_end(source) && self.peek(source) != '*' {
                                        self.advance(source);
                                    }
                                    if let Some(closing_c) = self.peek_next(source) {
                                        if closing_c == '/' {
                                            self.current += 2;
                                            break;
                                        } else {
                                            // Have not reached the end of the comment
                                            self.advance(source);
                                        }
                                    } else {
                                        return Some(format!(
                                            "Expected comment opened at {}, to close",
                                            self.start
                                        ));
                                    }
                                }
                            }
                            _ => {
                                // Not a comment
                                return None;
                            }
                        }
                    }
                    None => {
                        // Is a slash
                        return None;
                    }
                }
            } else {
                // Hit something that is not a whitespace
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn skip_whitespace() {
        let mut tokenizer = Tokenizer::new();
        let source = "    ";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 4);
        assert!(output.is_none());

        let mut tokenizer = Tokenizer::new();
        let source = "\n\n\n";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 3);
        assert_eq!(tokenizer.line, 4);
        assert!(output.is_none());
    }
    #[test]
    fn skip_comments() {
        let mut tokenizer = Tokenizer::new();
        let source = "//wow something here";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 20);
        assert!(output.is_none());

        let mut tokenizer = Tokenizer::new();
        let source = "//wow something here\n";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 21);
        assert_eq!(tokenizer.line, 2);
        assert!(output.is_none());

        let mut tokenizer = Tokenizer::new();
        let source = "/* wow a comment */\n";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 20);
        assert_eq!(tokenizer.line, 2);
        assert!(output.is_none());

        let mut tokenizer = Tokenizer::new();
        let source = "/** wow a funny * comment */\n";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 29);
        assert_eq!(tokenizer.line, 2);
        assert!(output.is_none());
    }
    #[test]
    fn not_closed_comment() {
        let mut tokenizer = Tokenizer::new();
        let source = "/** wow a funny * comment";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.skip_whitespace_and_comments(&source);
        assert_eq!(tokenizer.current, 25);
        assert!(output.is_some());
        assert_eq!(output.unwrap(), "Expected comment opened at 0, to close");
    }
    #[test]
    fn scan_whitespace() {
        let mut tokenizer = Tokenizer::new();
        let source = "  \n  \n";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_ok());
        assert!(matches!(output.unwrap()._type, TokenType::EOF));

        let mut tokenizer = Tokenizer::new();
        let source = "// wow funny \n";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert_eq!(tokenizer.line, 2);
        assert!(output.is_ok());
        assert!(matches!(output.unwrap()._type, TokenType::EOF));
    }
    #[test]
    fn scan_not_closed_comment() {
        let mut tokenizer = Tokenizer::new();
        let source = "/** wow a funny * comment";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_err());
        assert_eq!(
            output.err().unwrap(),
            "Expected comment opened at 0, to close"
        );
    }
}
