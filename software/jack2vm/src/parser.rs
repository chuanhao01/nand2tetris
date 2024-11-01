use crate::{ReservedKeywords, Symbols, Token, TokenType};

type ParserReturn = Result<(), String>;

struct Parser {
    current: usize,
    ast: Vec<String>,
}
impl Parser {
    fn new() -> Self {
        Self {
            ast: Vec::new(),
            current: 0,
        }
    }
    fn parse(&mut self, source: &[char], tokens: &[Token]) -> Result<String, String> {
        // Returns XML string to write to file
        let token = &tokens[self.current];
        if let TokenType::Keyword(ReservedKeywords::Class) = token._type {
            self.class(source, tokens)?;
        } else {
            return Err(format!(
                "Expected to start with class, got {}",
                token.get_source(source)
            ));
        }
        Ok(self.ast.join("\n"))
    }
    fn class(&mut self, source: &[char], tokens: &[Token]) -> ParserReturn {
        self.ast.push("<class>".to_string());

        // Consume class
        let token = self.advance(tokens, source)?;
        if !matches!(token._type, TokenType::Keyword(ReservedKeywords::Class)) {
            return Err(Self::check_token_type(
                &token,
                TokenType::Keyword(ReservedKeywords::Class),
                source,
            ));
        }
        self.push_keyword(&token, source);

        // Consume className
        let token = self.advance(tokens, source)?;
        if !matches!(token._type, TokenType::Identifier) {
            return Err(Self::check_token_type(
                &token,
                TokenType::Identifier,
                source,
            ));
        }

        // Consume '{'
        let token = self.advance(tokens, source)?;
        if !matches!(token._type, TokenType::Symbol(Symbols::LeftBrace)) {
            return Err(Self::check_token_type(
                &token,
                TokenType::Symbol(Symbols::LeftBrace),
                source,
            ));
        }

        self.ast.push("</class>".to_string());
        Ok(())
    }
    fn push_keyword(&mut self, token: &Token, source: &[char]) {
        self.ast.push("<keyword>".to_string());
        self.ast.push(token.get_source(source));
        self.ast.push("</keyword>".to_string());
    }
    fn push_terminal(&mut self, token: &Token, source: &[char]) {
        let _type = match token._type {
            TokenType::Identifier => "identifier",
            TokenType::Integer(_) => "integerConstant",
            TokenType::Keyword(_) => "keyword",
            TokenType::String => "stringConstant",
            TokenType::Symbol(_) => "symbol",
            TokenType::EOF => panic!("Should not be pushing EOF terminal"),
        };
        self.ast.push(format!("<{}>", _type));
        // self.ast.push()
        self.ast.push(format!("</{}>", _type));
    }

    fn advance(&mut self, tokens: &[Token], source: &[char]) -> Result<Token, String> {
        self.current += 1;
        if self.is_at_end(tokens) {
            Err(Self::error_unexpected_end(
                // Might be a bug, where current is moved forward more
                &tokens[self.current - 2],
                source,
            ))
        } else {
            // return the previous token
            let token = tokens[self.current - 1].clone();
            Ok(token)
        }
    }
    fn is_at_end(&mut self, tokens: &[Token]) -> bool {
        self.current >= tokens.len()
    }
    fn error_unexpected_end(token: &Token, source: &[char]) -> String {
        format!(
            "Unexpected end at {}, on line {}",
            token.get_source(source),
            token.line
        )
    }
    fn check_token_type(token: &Token, _type: TokenType, source: &[char]) -> String {
        format!(
            "Expected {:?}, got {} on line {}",
            _type,
            token.get_source(source),
            token.line
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_expected_token_string() {
        let source = "class".chars().collect::<Vec<char>>();
        let token = &Token {
            _type: TokenType::Keyword(ReservedKeywords::Class),
            start: 0,
            length: 5,
            line: 2,
        };
        let output =
            Parser::check_token_type(token, TokenType::Keyword(ReservedKeywords::Do), &source);
        println!("{:?}", output);
        assert_eq!(output, "Expected Keyword(Do), got class on line 2");
    }
}
