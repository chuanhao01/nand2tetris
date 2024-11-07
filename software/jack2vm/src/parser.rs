use crate::{ReservedKeywords, Symbols, Token, TokenType, Tokenizer};

pub type ParserReturn = Result<(), String>;

// Forbidden arts
// Used to help writing code that check and consumes a terminal token
// Creates:
//
// match token._type {
//     TokenType::Symbol(Symbols::RightBrace) => {}
//     _ => {
//         return Err(Self::error_expected_token_type(
//             &token,
//             &[TokenType::Symbol(Symbols::RightBrace)],
//             source,
//         ));
//     }
// }
// self.push_terminal(&token, source);
macro_rules! consume_single_terminal_token {
    ($self:ident, $token:ident, $_type_p:pat, $_type_e:expr, $source:ident) => {
        match $token._type {
            $_type_p => {}
            _ => {
                return Err(Parser::error_expected_token_type(
                    &$token,
                    &[$_type_e],
                    $source,
                ));
            }
        }
        $self.push_terminal(&$token, $source);
    };
}

/// Funny design decision
/// For the tokenizer we had to worry about over advancing and peeking
/// But since we have EOF token at the end
/// We can always just peek and check, because if we encounter the EOF before we need it, it throws an error
/// And if the last token is not EOF, it also means we did not parse correctly
///
/// Example where this needs to be considered is that when peeking to check if we need to consume 0 or more tokens,
/// we do no need to worry about not having any more tokens to consume
#[derive(Default)]
pub struct Parser {
    current: usize,
    ast: Vec<String>,
    class_name: String,
}
impl Parser {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn parse(source: &str) -> Result<String, String> {
        let tokens = Tokenizer::generate_tokens(source)?;
        // Debug show tokens
        #[cfg(feature = "debug")]
        {
            println!("{:?}", tokens);
            println!();
        }
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        parser.parse_tokens(&tokens, &source)
    }
    fn parse_tokens(&mut self, tokens: &[Token], source: &[char]) -> Result<String, String> {
        // Returns XML string to write to file
        let token = &tokens[self.current];
        if let TokenType::Keyword(ReservedKeywords::Class) = token._type {
            self.class(tokens, source)?;
        } else {
            return Err(format!(
                "Expected to start with class, got {}",
                token.get_source(source)
            ));
        }
        Ok(self.ast.join("\n"))
    }
    fn class(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<class>".to_string());

        // Consume class
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::Class),
            TokenType::Keyword(ReservedKeywords::Class),
            source
        );

        // Consume className, ignored
        self.identifier(tokens, source)?;
        let token = &tokens[self.current - 1];
        self.class_name = token.get_source(source);

        // Consume '{'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftBrace),
            TokenType::Symbol(Symbols::LeftBrace),
            source
        );

        // Check for 0 or more classVarDec
        while let TokenType::Keyword(ReservedKeywords::Static | ReservedKeywords::Field) =
            self.peek(tokens)._type
        {
            self.class_var_dec(tokens, source)?;
        }

        // subroutineDec*
        while let TokenType::Keyword(
            ReservedKeywords::Constructor | ReservedKeywords::Function | ReservedKeywords::Method,
        ) = self.peek(tokens)._type
        {
            self.subroutine_dec(tokens, source)?;
        }

        // '}'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightBrace),
            TokenType::Symbol(Symbols::RightBrace),
            source
        );

        self.ast.push("</class>".to_string());
        Ok(())
    }
    fn class_var_dec(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<classVarDec>".to_string());

        // Consume ('static' | 'field')
        let token = self.advance(tokens, source)?;
        if !matches!(
            token._type,
            TokenType::Keyword(ReservedKeywords::Static | ReservedKeywords::Field)
        ) {
            return Err(Self::error_expected_token_type(
                &token,
                &[
                    TokenType::Keyword(ReservedKeywords::Static),
                    TokenType::Keyword(ReservedKeywords::Field),
                ],
                source,
            ));
        }
        self.push_terminal(&token, source);

        // Pass to type
        self._type(tokens, source)?;

        // Consume varName, ignored
        self.identifier(tokens, source)?;

        // (',' varName)*
        while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
            // ','
            let token = self.advance(tokens, source)?;
            self.push_terminal(&token, source);
            // varName
            self.identifier(tokens, source)?;
        }

        // ';'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::SemiColon),
            TokenType::Symbol(Symbols::SemiColon),
            source
        );

        self.ast.push("</classVarDec>".to_string());
        Ok(())
    }
    fn subroutine_dec(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<subroutineDec>".to_string());

        // Consume ('constructor' | 'function' | 'method')
        let token = self.advance(tokens, source)?;
        match token._type {
            TokenType::Keyword(
                ReservedKeywords::Constructor
                | ReservedKeywords::Function
                | ReservedKeywords::Method,
            ) => {}
            _ => {
                return Err(Self::error_expected_token_type(
                    &token,
                    &[
                        TokenType::Keyword(ReservedKeywords::Constructor),
                        TokenType::Keyword(ReservedKeywords::Function),
                        TokenType::Keyword(ReservedKeywords::Method),
                    ],
                    source,
                ));
            }
        }
        self.push_terminal(&token, source);

        // ('void' | type)
        let token = self.peek(tokens);
        match token._type {
            TokenType::Keyword(ReservedKeywords::Void) => {
                self.push_terminal(&token, source);
                self.advance(tokens, source)?;
            }
            _ => {
                self._type(tokens, source)?;
            }
        }

        // subroutineName, identifier, ignored
        self.identifier(tokens, source)?;

        // '('
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftParam),
            TokenType::Symbol(Symbols::LeftParam),
            source
        );

        // parameterList
        self.parameter_list(tokens, source)?;

        // ')'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightParam),
            TokenType::Symbol(Symbols::RightParam),
            source
        );

        // subroutineBody
        self.subroutine_body(tokens, source)?;

        self.ast.push("</subroutineDec>".to_string());
        Ok(())
    }
    fn subroutine_body(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<subroutineBody>".to_string());

        // '{'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftBrace),
            TokenType::Symbol(Symbols::LeftBrace),
            source
        );

        // varDec*
        while let TokenType::Keyword(ReservedKeywords::Var) = self.peek(tokens)._type {
            self.var_dec(tokens, source)?;
        }

        // statements
        self.statements(tokens, source)?;

        // '}'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightBrace),
            TokenType::Symbol(Symbols::RightBrace),
            source
        );

        self.ast.push("</subroutineBody>".to_string());
        Ok(())
    }
    fn var_dec(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<varDec>".to_string());

        // 'var'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::Var),
            TokenType::Keyword(ReservedKeywords::Var),
            source
        );

        // type
        self._type(tokens, source)?;

        // varName, ignored
        self.identifier(tokens, source)?;

        // (',' varName)*
        while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
            // ','
            let token = self.advance(tokens, source)?;
            self.push_terminal(&token, source);
            // varName
            self.identifier(tokens, source)?;
        }

        // ';'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::SemiColon),
            TokenType::Symbol(Symbols::SemiColon),
            source
        );

        self.ast.push("</varDec>".to_string());
        Ok(())
    }
    fn parameter_list(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<parameterList>".to_string());
        // ()?
        if let TokenType::Keyword(
            ReservedKeywords::Int | ReservedKeywords::Char | ReservedKeywords::Boolean,
        )
        | TokenType::Identifier = self.peek(tokens)._type
        {
            self._type(tokens, source)?;
            self.identifier(tokens, source)?;

            // (',' type varName)*
            while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
                // ','
                let token = self.advance(tokens, source)?;
                self.push_terminal(&token, source);
                // type
                self._type(tokens, source)?;
                // varName
                self.identifier(tokens, source)?;
            }
        }

        self.ast.push("</parameterList>".to_string());
        Ok(())
    }
    fn _type(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        let token = self.advance(tokens, source)?;

        match token._type {
            TokenType::Keyword(
                ReservedKeywords::Int | ReservedKeywords::Char | ReservedKeywords::Boolean,
            )
            | TokenType::Identifier => {
                // Consume the Reserved Keyword
                self.push_terminal(&token, source);
                Ok(())
            }
            _ => Err(Self::error_expected_token_type(
                &token,
                &[
                    TokenType::Keyword(ReservedKeywords::Int),
                    TokenType::Keyword(ReservedKeywords::Char),
                    TokenType::Keyword(ReservedKeywords::Boolean),
                    TokenType::Identifier,
                ],
                source,
            )),
        }
    }
    fn identifier(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        // Consume identifier, ignored
        let token = self.advance(tokens, source)?;
        match token._type {
            TokenType::Identifier => {}
            _ => {
                return Err(Self::error_expected_token_type(
                    &token,
                    &[TokenType::Identifier],
                    source,
                ))
            }
        }
        self.push_terminal(&token, source);

        Ok(())
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
        self.ast.push(token.get_source(source));
        self.ast.push(format!("</{}>", _type));
    }

    // Statements
    // NOTE:
    // Could combine with self.statement
    // Match on every type, then call it
    fn statements(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<statements>".to_string());
        loop {
            let token = self.peek(tokens);
            match token._type {
                TokenType::Keyword(
                    ReservedKeywords::Let
                    | ReservedKeywords::If
                    | ReservedKeywords::While
                    | ReservedKeywords::Do
                    | ReservedKeywords::Return,
                ) => {
                    self.statement(tokens, source)?;
                }
                _ => break,
            }
        }

        self.ast.push("</statements>".to_string());
        Ok(())
    }
    fn statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        let token = self.peek(tokens);
        match token._type {
            TokenType::Keyword(ReservedKeywords::Let) => {
                self.let_statement(tokens, source)?;
            }
            TokenType::Keyword(ReservedKeywords::If) => {
                self.if_statement(tokens, source)?;
            }
            TokenType::Keyword(ReservedKeywords::While) => {
                self.while_statement(tokens, source)?;
            }
            TokenType::Keyword(ReservedKeywords::Do) => {
                self.do_statement(tokens, source)?;
            }
            TokenType::Keyword(ReservedKeywords::Return) => {
                self.return_statement(tokens, source)?
            }
            _ => {
                return Err(Self::error_expected_token_type(
                    &token,
                    &[
                        TokenType::Keyword(ReservedKeywords::Let),
                        TokenType::Keyword(ReservedKeywords::If),
                        TokenType::Keyword(ReservedKeywords::While),
                        TokenType::Keyword(ReservedKeywords::Do),
                        TokenType::Keyword(ReservedKeywords::Return),
                    ],
                    source,
                ));
            }
        }
        Ok(())
    }
    fn let_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<letStatement>".to_string());

        // 'let'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::Let),
            TokenType::Keyword(ReservedKeywords::Let),
            source
        );

        // varName
        self.identifier(tokens, source)?;

        // ('[' expression ']')?
        if let TokenType::Symbol(Symbols::LeftBracket) = self.peek(tokens)._type {
            // '['
            let token = self.advance(tokens, source)?;
            consume_single_terminal_token!(
                self,
                token,
                TokenType::Symbol(Symbols::LeftBracket),
                TokenType::Symbol(Symbols::LeftBracket),
                source
            );

            // expression
            self.expression(tokens, source)?;

            // ']'
            let token = self.advance(tokens, source)?;
            consume_single_terminal_token!(
                self,
                token,
                TokenType::Symbol(Symbols::RightBracket),
                TokenType::Symbol(Symbols::RightBracket),
                source
            );
        }

        // '='
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::Equal),
            TokenType::Symbol(Symbols::Equal),
            source
        );

        self.expression(tokens, source)?;

        // ';'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::SemiColon),
            TokenType::Symbol(Symbols::SemiColon),
            source
        );

        self.ast.push("</letStatement>".to_string());
        Ok(())
    }
    fn if_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<ifStatement>".to_string());

        // 'if'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::If),
            TokenType::Keyword(ReservedKeywords::If),
            source
        );

        // '('
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftParam),
            TokenType::Symbol(Symbols::LeftParam),
            source
        );

        self.expression(tokens, source)?;

        // ')'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightParam),
            TokenType::Symbol(Symbols::RightParam),
            source
        );

        // '{'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftBrace),
            TokenType::Symbol(Symbols::LeftBrace),
            source
        );

        self.statements(tokens, source)?;

        // '}'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightBrace),
            TokenType::Symbol(Symbols::RightBrace),
            source
        );

        // ()?
        if let TokenType::Keyword(ReservedKeywords::Else) = self.peek(tokens)._type {
            // 'else'
            let token = self.advance(tokens, source)?;
            self.push_terminal(&token, source);

            // '{'
            let token = self.advance(tokens, source)?;
            consume_single_terminal_token!(
                self,
                token,
                TokenType::Symbol(Symbols::LeftBrace),
                TokenType::Symbol(Symbols::LeftBrace),
                source
            );

            self.statements(tokens, source)?;

            // '}'
            let token = self.advance(tokens, source)?;
            consume_single_terminal_token!(
                self,
                token,
                TokenType::Symbol(Symbols::RightBrace),
                TokenType::Symbol(Symbols::RightBrace),
                source
            );
        }

        self.ast.push("</ifStatement>".to_string());
        Ok(())
    }
    fn while_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<whileStatement>".to_string());
        // 'while'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::While),
            TokenType::Keyword(ReservedKeywords::While),
            source
        );

        // '('
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftParam),
            TokenType::Symbol(Symbols::LeftParam),
            source
        );

        self.expression(tokens, source)?;

        // ')'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightParam),
            TokenType::Symbol(Symbols::RightParam),
            source
        );

        // '{'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftBrace),
            TokenType::Symbol(Symbols::LeftBrace),
            source
        );

        self.statements(tokens, source)?;

        // '}'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightBrace),
            TokenType::Symbol(Symbols::RightBrace),
            source
        );

        self.ast.push("</whileStatement>".to_string());
        Ok(())
    }
    fn do_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<doStatement>".to_string());

        // 'do'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::Do),
            TokenType::Keyword(ReservedKeywords::Do),
            source
        );

        self.subroutine_call(tokens, source)?;

        // ';'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::SemiColon),
            TokenType::Symbol(Symbols::SemiColon),
            source
        );

        self.ast.push("</doStatement>".to_string());
        Ok(())
    }
    fn return_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<returnStatement>".to_string());

        // 'return'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Keyword(ReservedKeywords::Return),
            TokenType::Keyword(ReservedKeywords::Return),
            source
        );

        // expression? ';'
        match self.peek(tokens)._type {
            TokenType::Symbol(Symbols::SemiColon) => {
                // Skip expression
            }
            _ => {
                // Consume expression
                self.expression(tokens, source)?;
            }
        }

        // ';'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::SemiColon),
            TokenType::Symbol(Symbols::SemiColon),
            source
        );

        self.ast.push("</returnStatement>".to_string());
        Ok(())
    }

    // Expressions
    fn expression(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<expression>".to_string());
        self.term(tokens, source)?;
        // (op term)*
        while let TokenType::Symbol(
            Symbols::Plus
            | Symbols::Minus
            | Symbols::Asterisk
            | Symbols::Slash
            | Symbols::And
            | Symbols::Or
            | Symbols::GreaterThan
            | Symbols::LessThan
            | Symbols::Equal,
        ) = self.peek(tokens)._type
        {
            let token = self.advance(tokens, source)?;
            self.push_terminal(&token, source);
            self.term(tokens, source)?;
        }
        self.ast.push("</expression>".to_string());
        Ok(())
    }
    fn subroutine_call(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        // subroutineName | className | varName
        self.identifier(tokens, source)?;
        match self.peek(tokens)._type {
            TokenType::Symbol(Symbols::LeftParam) => {
                // '('
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::LeftParam),
                    TokenType::Symbol(Symbols::LeftParam),
                    source
                );

                self.expression_list(tokens, source)?;

                // ')'
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::RightParam),
                    TokenType::Symbol(Symbols::RightParam),
                    source
                );
            }
            TokenType::Symbol(Symbols::Dot) => {
                // '.'
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::Dot),
                    TokenType::Symbol(Symbols::Dot),
                    source
                );

                // subroutineName
                self.identifier(tokens, source)?;

                // '('
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::LeftParam),
                    TokenType::Symbol(Symbols::LeftParam),
                    source
                );

                self.expression_list(tokens, source)?;

                // ')'
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::RightParam),
                    TokenType::Symbol(Symbols::RightParam),
                    source
                );
            }
            _ => {
                let token = &self.peek(tokens);
                return Err(Self::error_expected_token_type(
                    token,
                    &[
                        TokenType::Symbol(Symbols::LeftParam),
                        TokenType::Symbol(Symbols::Dot),
                    ],
                    source,
                ));
            }
        }
        Ok(())
    }
    fn term(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<term>".to_string());
        let token = self.peek(tokens);
        match token._type {
            TokenType::Keyword(
                ReservedKeywords::True
                | ReservedKeywords::False
                | ReservedKeywords::Null
                | ReservedKeywords::This,
            )
            | TokenType::String
            | TokenType::Integer(_) => {
                self.push_terminal(&token, source);
                self.advance(tokens, source)?;
            }
            TokenType::Identifier => {
                // Decide between varName, varName '[' and subroutine_call
                // LL(2)
                match self.peek_n(1, tokens, source) {
                    Some(next_token) => {
                        match next_token._type {
                            TokenType::Symbol(Symbols::LeftBracket) => {
                                self.identifier(tokens, source)?;
                                // '['
                                let token = self.advance(tokens, source)?;
                                consume_single_terminal_token!(
                                    self,
                                    token,
                                    TokenType::Symbol(Symbols::LeftBracket),
                                    TokenType::Symbol(Symbols::LeftBracket),
                                    source
                                );

                                // expression
                                self.expression(tokens, source)?;

                                // ']'
                                let token = self.advance(tokens, source)?;
                                consume_single_terminal_token!(
                                    self,
                                    token,
                                    TokenType::Symbol(Symbols::RightBracket),
                                    TokenType::Symbol(Symbols::RightBracket),
                                    source
                                );
                            }
                            TokenType::Symbol(Symbols::LeftBrace | Symbols::Dot) => {
                                self.subroutine_call(tokens, source)?;
                            }
                            _ => {
                                // Just varName
                                self.identifier(tokens, source)?;
                            }
                        }
                    }
                    None => {
                        // Just varName
                        self.identifier(tokens, source)?;
                    }
                }
            }
            TokenType::Symbol(Symbols::LeftParam) => {
                // '('
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::LeftParam),
                    TokenType::Symbol(Symbols::LeftParam),
                    source
                );

                self.expression(tokens, source)?;

                // ')'
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::RightParam),
                    TokenType::Symbol(Symbols::RightParam),
                    source
                );
            }
            TokenType::Symbol(Symbols::Minus | Symbols::Tilde) => {
                self.push_terminal(&token, source);
                self.advance(tokens, source)?;
                self.term(tokens, source)?;
            }
            _ => {
                return Err(format!(
                    "Expected a term, got {} on line {}",
                    token.get_source(source),
                    token.line
                ));
            }
        }
        self.ast.push("</term>".to_string());
        Ok(())
    }
    fn expression_list(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.ast.push("<expressionList>".to_string());
        // (expression (',' expression)*)?
        match self.peek(tokens)._type {
            TokenType::Identifier
            | TokenType::Integer(_)
            | TokenType::String
            | TokenType::Keyword(
                ReservedKeywords::This
                | ReservedKeywords::True
                | ReservedKeywords::False
                | ReservedKeywords::Null,
            )
            | TokenType::Symbol(Symbols::Minus | Symbols::Tilde | Symbols::LeftParam) => {
                self.expression(tokens, source)?;

                // (',' expression)*
                while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
                    // ','
                    let token = self.advance(tokens, source)?;
                    self.push_terminal(&token, source);

                    self.expression(tokens, source)?;
                }
            }
            _ => {}
        }
        self.ast.push("</expressionList>".to_string());
        Ok(())
    }

    fn advance(&mut self, tokens: &[Token], source: &[char]) -> Result<Token, String> {
        self.current += 1;
        if self.current > tokens.len() {
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
    fn peek(&self, tokens: &[Token]) -> Token {
        tokens[self.current].clone()
    }
    fn peek_n(&self, n: usize, tokens: &[Token], source: &[char]) -> Option<Token> {
        if self.current + n >= tokens.len() {
            None
        } else {
            Some(tokens[self.current + n].clone())
        }
    }
    fn error_unexpected_end(token: &Token, source: &[char]) -> String {
        format!(
            "Unexpected end at {}, on line {}",
            token.get_source(source),
            token.line
        )
    }
    pub fn error_expected_token_type(
        token: &Token,
        _types: &[TokenType],
        source: &[char],
    ) -> String {
        format!(
            "Expected {}, got {} on line {}",
            _types
                .iter()
                .map(|_type| format!("{:?}", _type))
                .collect::<Vec<String>>()
                .join(" | "),
            token.get_source(source),
            token.line
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Tokenizer;

    use super::*;
    #[test]
    fn custom() {
        let source = "class TEstClassName { static WowName name1, name2;\nfield NmmSw wowname1;\nfunction funnyClass wow_method(int wiw, class2 damn, nans bob){\nvar int a;let a=13;} }"
            .to_string();
        let tokens = Tokenizer::generate_tokens(&source).unwrap();
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        let output = parser.parse_tokens(&tokens, &source);
        println!("{:?}", tokens);
        println!("{:?}", output);
        println!("{:?}", parser.ast);
        assert!(false);
    }

    #[test]
    fn empty_class() {
        let source = "class TEstClassName {}".to_string();
        let tokens = Tokenizer::generate_tokens(&source).unwrap();
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        let output = parser.parse_tokens(&tokens, &source);
        assert!(output.is_ok());
    }
    #[test]
    fn broken_class() {
        let source = "class TEstClassName {".to_string();
        let tokens = Tokenizer::generate_tokens(&source).unwrap();
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        let output = parser.parse_tokens(&tokens, &source);
        assert!(output.is_err());

        let source = "class let {}".to_string();
        let tokens = Tokenizer::generate_tokens(&source).unwrap();
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        let output = parser.parse_tokens(&tokens, &source);
        assert!(output.is_err());
    }
    #[test]
    fn class_with_vars() {
        let source =
            "class TEstClassName { static int field1, field2;\nfield someClass name1;}".to_string();
        let tokens = Tokenizer::generate_tokens(&source).unwrap();
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        let output = parser.parse_tokens(&tokens, &source);
        assert!(output.is_ok());
    }
    #[test]
    fn error_expected_token_string() {
        let source = "class".chars().collect::<Vec<char>>();
        let token = &Token {
            _type: TokenType::Keyword(ReservedKeywords::Class),
            start: 0,
            length: 5,
            line: 2,
        };
        let output = Parser::error_expected_token_type(
            token,
            &[TokenType::Keyword(ReservedKeywords::Do)],
            &source,
        );
        println!("{:?}", output);
        assert_eq!(output, "Expected Keyword(Do), got class on line 2");

        let source = "class".chars().collect::<Vec<char>>();
        let token = &Token {
            _type: TokenType::Keyword(ReservedKeywords::Class),
            start: 0,
            length: 5,
            line: 2,
        };
        let output = Parser::error_expected_token_type(
            token,
            &[
                TokenType::Keyword(ReservedKeywords::Let),
                TokenType::Symbol(Symbols::And),
            ],
            &source,
        );
        println!("{:?}", output);
        assert_eq!(
            output,
            "Expected Keyword(Let) | Symbol(And), got class on line 2"
        );
    }
    #[test]
    fn push_terminal_ast() {
        let source = "let".chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::Keyword(ReservedKeywords::Let),
            start: 0,
            length: 3,
            line: 1,
        };
        let mut parser = Parser::new();
        parser.push_terminal(&token, &source);
        assert_eq!(parser.ast.len(), 3);
        assert_eq!(parser.ast[0], "<keyword>");
        assert_eq!(parser.ast[1], "let");
        assert_eq!(parser.ast[2], "</keyword>");

        let source = "\"funny string\"".chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::String,
            start: 0,
            length: 14,
            line: 1,
        };
        let mut parser = Parser::new();
        parser.push_terminal(&token, &source);
        assert_eq!(parser.ast.len(), 3);
        assert_eq!(parser.ast[0], "<stringConstant>");
        assert_eq!(parser.ast[1], "funny string");
        assert_eq!(parser.ast[2], "</stringConstant>");
    }
}
