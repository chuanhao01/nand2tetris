use crate::{ReservedKeywords, Symbols, Token, TokenType};

pub struct Tokenizer {
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
    pub fn generate_tokens(source: &str) -> Result<Vec<Token>, String> {
        let source = source.chars().collect::<Vec<char>>();
        let mut tokens: Vec<Token> = Vec::new();
        let mut tokenizer = Self::new();
        loop {
            let token = tokenizer.scan_token(&source);
            match token {
                Ok(token) => {
                    if matches!(token._type, TokenType::EOF) {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }
    fn scan_token(&mut self, source: &[char]) -> Result<Token, String> {
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
        let c = self.advance(source).unwrap();
        if Self::is_alpha(c) {
            self.identifier(source)
        } else if c.is_numeric() {
            self.integer(source)
        } else if c == '\"' {
            self.string(source)
        } else {
            match c {
                '{' => Ok(self.make_token(TokenType::Symbol(Symbols::LeftBrace))),
                '}' => Ok(self.make_token(TokenType::Symbol(Symbols::RightBrace))),
                '(' => Ok(self.make_token(TokenType::Symbol(Symbols::LeftParam))),
                ')' => Ok(self.make_token(TokenType::Symbol(Symbols::RightParam))),
                '[' => Ok(self.make_token(TokenType::Symbol(Symbols::LeftBracket))),
                ']' => Ok(self.make_token(TokenType::Symbol(Symbols::RightBracket))),
                '.' => Ok(self.make_token(TokenType::Symbol(Symbols::Dot))),
                ',' => Ok(self.make_token(TokenType::Symbol(Symbols::Comma))),
                ';' => Ok(self.make_token(TokenType::Symbol(Symbols::SemiColon))),
                '+' => Ok(self.make_token(TokenType::Symbol(Symbols::Plus))),
                '-' => Ok(self.make_token(TokenType::Symbol(Symbols::Minus))),
                '*' => Ok(self.make_token(TokenType::Symbol(Symbols::Asterisk))),
                '/' => Ok(self.make_token(TokenType::Symbol(Symbols::Slash))),
                '&' => Ok(self.make_token(TokenType::Symbol(Symbols::And))),
                '|' => Ok(self.make_token(TokenType::Symbol(Symbols::Or))),
                '<' => Ok(self.make_token(TokenType::Symbol(Symbols::LessThan))),
                '>' => Ok(self.make_token(TokenType::Symbol(Symbols::GreaterThan))),
                '=' => Ok(self.make_token(TokenType::Symbol(Symbols::Equal))),
                '~' => Ok(self.make_token(TokenType::Symbol(Symbols::Tilde))),
                _ => Err(format!("Unknown token at {}, {}", self.start, c)),
            }
        }
    }
    fn is_at_end(&mut self, source: &[char]) -> bool {
        self.current >= source.len()
    }
    fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    fn identifier(&mut self, source: &[char]) -> Result<Token, String> {
        // Take chars if its still alpha or numeric
        while !self.is_at_end(source)
            && (Self::is_alpha(self.peek(source)) || self.peek(source).is_numeric())
        {
            self.advance(source);
        }
        let identifier = source[self.start..self.current].iter().collect::<String>();
        match identifier.as_str() {
            "class" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Class))),
            "constructor" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Constructor))),
            "function" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Function))),
            "method" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Method))),
            "field" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Field))),
            "static" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Static))),
            "var" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Var))),
            "int" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Int))),
            "char" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Char))),
            "boolean" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Boolean))),
            "void" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Void))),
            "true" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::True))),
            "false" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::False))),
            "null" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Null))),
            "this" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::This))),
            "let" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Let))),
            "do" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Do))),
            "if" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::If))),
            "else" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Else))),
            "while" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::While))),
            "return" => Ok(self.make_token(TokenType::Keyword(ReservedKeywords::Return))),
            _ => Ok(self.make_token(TokenType::Identifier)),
        }
    }
    fn integer(&mut self, source: &[char]) -> Result<Token, String> {
        // Take chars if its  numeric
        while !self.is_at_end(source) && self.peek(source).is_numeric() {
            self.advance(source);
        }
        match source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<usize>()
        {
            Ok(value) => {
                if value > 32767 {
                    Err(format!(
                        "Integer {} at {} is too large, should be within 0 and 32767",
                        value, self.start
                    ))
                } else {
                    Ok(self.make_token(TokenType::Integer(value)))
                }
            }
            Err(e) => Err(format!(
                "Failed to tokenize integer at {}, {}",
                self.start, e
            )),
        }
    }
    fn string(&mut self, source: &[char]) -> Result<Token, String> {
        while !self.is_at_end(source) && self.peek(source) != '\"' {
            self.advance(source);
        }
        if self.is_at_end(source) {
            return Err(format!(
                "String opened at {}, not closed at {}",
                self.start, self.current
            ));
        }
        // Consume the final "
        self.advance(source);
        Ok(self.make_token(TokenType::String))
    }
    fn make_token(&self, _type: TokenType) -> Token {
        Token::new(_type, self.start, self.current - self.start, self.line)
    }
    // Moves pointer forward, returns the current char
    fn advance(&mut self, source: &[char]) -> Option<char> {
        self.current += 1;
        if self.current > source.len() {
            None
        } else {
            Some(source[self.current - 1])
        }
    }
    fn peek(&self, source: &[char]) -> char {
        // Will never fail, as calls to peek should be on a valid character
        source[self.current]
    }
    fn peek_n(&self, source: &[char], n: usize) -> Option<char> {
        if self.current + n >= source.len() {
            None
        } else {
            Some(source[self.current + n])
        }
    }
    fn peek_next(&self, source: &[char]) -> Option<char> {
        self.peek_n(source, 1)
    }
    fn skip_whitespace_and_comments(&mut self, source: &[char]) -> Option<String> {
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
    #[test]
    fn scan_unknown_token() {
        let mut tokenizer = Tokenizer::new();
        let source = "%";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_err());
        assert_eq!(output.err().unwrap(), "Unknown token at 0, %");
    }
    #[test]
    fn scan_string_token() {
        let mut tokenizer = Tokenizer::new();
        let source = "\"hello world there!\"";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_ok());
        assert!(matches!(output.unwrap()._type, TokenType::String));
    }
    #[test]
    fn scan_string_not_closed() {
        let mut tokenizer = Tokenizer::new();
        let source = "\"hello world there!";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_err());
        assert_eq!(
            output.err().unwrap(),
            "String opened at 0, not closed at 19"
        );
    }
    #[test]
    fn scan_integer() {
        let mut tokenizer = Tokenizer::new();
        let source = "1234";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_ok());
        assert!(matches!(output.unwrap()._type, TokenType::Integer(1234)));
    }
    #[test]
    fn scan_overflow_integer() {
        let mut tokenizer = Tokenizer::new();
        let source = "1234567";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_err());
        assert_eq!(
            output.err().unwrap(),
            "Integer 1234567 at 0 is too large, should be within 0 and 32767"
        );
    }
    #[test]
    fn scan_keyword() {
        let mut tokenizer = Tokenizer::new();
        let source = "class";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_ok());
        assert!(matches!(
            output.unwrap()._type,
            TokenType::Keyword(ReservedKeywords::Class)
        ));

        let mut tokenizer = Tokenizer::new();
        let source = "boolean";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_ok());
        assert!(matches!(
            output.unwrap()._type,
            TokenType::Keyword(ReservedKeywords::Boolean)
        ));
    }
    #[test]
    fn scan_identifier() {
        let mut tokenizer = Tokenizer::new();
        let source = "abc";
        let source = source.chars().collect::<Vec<char>>();
        let output = tokenizer.scan_token(&source);
        assert!(output.is_ok());
        assert!(matches!(output.unwrap()._type, TokenType::Identifier));
    }
    #[test]
    fn generate_tokens_identifier() {
        let source = "abc".to_string();
        let output = Tokenizer::generate_tokens(&source);
        assert!(output.is_ok());
        assert_eq!(output.clone().unwrap().len(), 2);
        assert!(matches!(
            output.clone().unwrap()[0]._type,
            TokenType::Identifier
        ));
        assert!(matches!(output.clone().unwrap()[1]._type, TokenType::EOF));
        assert_eq!(
            "abc",
            output.unwrap()[0].get_source(&source.chars().collect::<Vec<char>>())
        )
    }
    #[test]
    fn generate_tokens_numbers() {
        let source = "-21".to_string();
        let output = Tokenizer::generate_tokens(&source);
        assert!(output.is_ok());
        assert_eq!(output.clone().unwrap().len(), 3);
        assert!(matches!(
            output.clone().unwrap()[0]._type,
            TokenType::Symbol(Symbols::Minus)
        ));
        assert!(matches!(
            output.clone().unwrap()[1]._type,
            TokenType::Integer(21)
        ));
    }
    #[test]
    fn generate_keywords() {
        let source = "class let boolean".to_string();
        let output = Tokenizer::generate_tokens(&source);
        assert!(output.is_ok());
        assert_eq!(output.clone().unwrap().len(), 4);
        assert!(matches!(
            output.clone().unwrap()[0]._type,
            TokenType::Keyword(ReservedKeywords::Class)
        ));
        assert!(matches!(output.clone().unwrap()[0].start, 0));
        assert!(matches!(output.clone().unwrap()[0].length, 5));
        assert!(matches!(
            output.clone().unwrap()[1]._type,
            TokenType::Keyword(ReservedKeywords::Let)
        ));
        assert!(matches!(
            output.clone().unwrap()[2]._type,
            TokenType::Keyword(ReservedKeywords::Boolean)
        ));
    }
}
