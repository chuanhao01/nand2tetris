use crate::{
    CodeGen, ReservedKeywords, Symbols, Token, TokenType, Tokenizer, VariableKind, VM_OPS,
};

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
pub struct Parser {
    current: usize,
    xml_ast: Vec<String>,
    class_name: Option<String>, // This is set when we look at the class dec
    pub code_gen: CodeGen,
}
#[allow(clippy::derivable_impls)]
impl Default for Parser {
    fn default() -> Self {
        Self {
            current: 0,
            xml_ast: Vec::default(),
            class_name: None,
            code_gen: CodeGen::default(),
        }
    }
}
#[derive(Debug)]
pub struct ParserCodeOutput {
    pub xml: String,
    pub vm: String,
}
impl Parser {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn parse(source: &str) -> Result<ParserCodeOutput, String> {
        let tokens = Tokenizer::generate_tokens(source)?;
        // Debug show tokens
        #[cfg(feature = "debug")]
        {
            println!("{:?}", tokens);
            println!();
        }
        let source = source.chars().collect::<Vec<char>>();
        let mut parser = Parser::new();
        let result = parser.parse_tokens(&tokens, &source);
        #[cfg(feature = "debug")]
        {
            println!(
                "{} class symbol ",
                parser.class_name.unwrap_or(String::from("empty class"))
            );
            println!("{:?}", parser.code_gen.class_symbol_table);
            println!();
        }
        result
    }
    fn parse_tokens(
        &mut self,
        tokens: &[Token],
        source: &[char],
    ) -> Result<ParserCodeOutput, String> {
        // Returns XML string to write to file
        if let TokenType::Keyword(ReservedKeywords::Class) = &tokens[self.current]._type {
            self.class(tokens, source)?;
        }
        if let TokenType::EOF = &tokens[self.current]._type {
            // Check if last token is EOF
            Ok(ParserCodeOutput {
                xml: self.xml_ast.join("\n"),
                vm: self.code_gen.gen_vm_code(),
            })
        } else {
            Err(String::from("Could not compile file at all, at the start"))
        }
    }
    fn class(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<class>".to_string());

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
        self.class_name = Some(token.get_source(source));

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

        self.xml_ast.push("</class>".to_string());
        Ok(())
    }
    fn class_var_dec(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<classVarDec>".to_string());

        // Consume ('static' | 'field')
        let token = self.advance(tokens, source)?;
        let variable_kind = &token;
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
        let variable_type = &tokens[self.current - 1];

        // Consume varName, ignored
        self.identifier(tokens, source)?;
        let name = tokens[self.current - 1].get_source(source);
        self.code_gen
            .insert_class_variable(name, variable_kind, variable_type, source)?;

        // (',' varName)*
        while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
            // ','
            let token = self.advance(tokens, source)?;
            self.push_terminal(&token, source);
            // varName
            self.identifier(tokens, source)?;
            let name = tokens[self.current - 1].get_source(source);
            self.code_gen
                .insert_class_variable(name, variable_kind, variable_type, source)?;
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

        self.xml_ast.push("</classVarDec>".to_string());
        Ok(())
    }
    fn subroutine_dec(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<subroutineDec>".to_string());

        // Consume ('constructor' | 'function' | 'method')
        let subroutine_kind_token = self.advance(tokens, source)?;
        self.code_gen.push_comment(format!(
            "start subroutine_dec, {}",
            subroutine_kind_token.line
        ));
        match subroutine_kind_token._type {
            TokenType::Keyword(
                ReservedKeywords::Constructor
                | ReservedKeywords::Function
                | ReservedKeywords::Method,
            ) => {}
            _ => {
                return Err(Self::error_expected_token_type(
                    &subroutine_kind_token,
                    &[
                        TokenType::Keyword(ReservedKeywords::Constructor),
                        TokenType::Keyword(ReservedKeywords::Function),
                        TokenType::Keyword(ReservedKeywords::Method),
                    ],
                    source,
                ));
            }
        }
        self.push_terminal(&subroutine_kind_token, source);

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
        let token = &tokens[self.current - 1];
        let subroutine_name = token.get_source(source);

        // '('
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::LeftParam),
            TokenType::Symbol(Symbols::LeftParam),
            source
        );

        // decalring a new subroutine
        self.code_gen
            .reset_subroutine_table(self.class_name.clone().unwrap());

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
        self.xml_ast.push("<subroutineBody>".to_string());

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
        // vmcode, declare the function
        self.code_gen
            .push_function(&self.class_name.clone().unwrap(), &subroutine_name);
        match subroutine_kind_token._type {
            TokenType::Keyword(ReservedKeywords::Constructor) => {
                self.code_gen.constructor_alloc();
            }
            TokenType::Keyword(ReservedKeywords::Method) => {
                self.code_gen.push_variable(&String::from("this"))?;
                self.code_gen.pop_pointer(0);
            }
            TokenType::Keyword(ReservedKeywords::Function) => {}
            _ => return Err(String::from("unexpected vmcodegen subroutine_dec")),
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

        self.xml_ast.push("</subroutineBody>".to_string());

        self.xml_ast.push("</subroutineDec>".to_string());
        #[cfg(feature = "debug")]
        {
            println!(
                "{}.{} symbol table",
                self.class_name.clone().unwrap(),
                subroutine_name
            );
            println!("{:?}", self.code_gen.subroutine_symbol_table);
            println!();
        }
        self.code_gen
            .push_comment(format!("end subroutine_dec, {}", tokens[self.current].line));
        Ok(())
    }
    fn var_dec(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<varDec>".to_string());

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
        let variable_type = &tokens[self.current - 1];

        // varName, ignored
        self.identifier(tokens, source)?;
        let name = tokens[self.current - 1].get_source(source);
        self.code_gen.insert_subroutine_variable(
            name,
            VariableKind::Local,
            variable_type,
            source,
        )?;

        // (',' varName)*
        while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
            // ','
            let token = self.advance(tokens, source)?;
            self.push_terminal(&token, source);
            // varName
            self.identifier(tokens, source)?;

            // variables declared on the same line have the same type
            let name = tokens[self.current - 1].get_source(source);
            self.code_gen.insert_subroutine_variable(
                name,
                VariableKind::Local,
                variable_type,
                source,
            )?;
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

        self.xml_ast.push("</varDec>".to_string());
        Ok(())
    }
    fn parameter_list(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<parameterList>".to_string());
        // ()?
        if let TokenType::Keyword(
            ReservedKeywords::Int | ReservedKeywords::Char | ReservedKeywords::Boolean,
        )
        | TokenType::Identifier = self.peek(tokens)._type
        {
            self._type(tokens, source)?;
            let variable_type = &tokens[self.current - 1];
            self.identifier(tokens, source)?;
            let name = tokens[self.current - 1].get_source(source);
            self.code_gen.insert_subroutine_variable(
                name,
                VariableKind::Argument,
                &variable_type,
                source,
            )?;

            // (',' type varName)*
            while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
                // ','
                let token = self.advance(tokens, source)?;
                self.push_terminal(&token, source);
                // type
                self._type(tokens, source)?;
                let variable_type = &tokens[self.current - 1];
                // varName
                self.identifier(tokens, source)?;
                let name = tokens[self.current - 1].get_source(source);
                self.code_gen.insert_subroutine_variable(
                    name,
                    VariableKind::Argument,
                    &variable_type,
                    source,
                )?;
            }
        }

        self.xml_ast.push("</parameterList>".to_string());
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
        self.xml_ast.push(format!("<{}>", _type));
        self.xml_ast.push(token.get_source(source));
        self.xml_ast.push(format!("</{}>", _type));
    }

    // Statements
    // NOTE:
    // Could combine with self.statement
    // Match on every type, then call it
    fn statements(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<statements>".to_string());
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

        self.xml_ast.push("</statements>".to_string());
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
        self.xml_ast.push("<letStatement>".to_string());

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
        let variable_token = &tokens[self.current - 1];
        let variable_name = &variable_token.get_source(source);

        let mut is_array = false;

        // ('[' expression ']')?
        if let TokenType::Symbol(Symbols::LeftBracket) = self.peek(tokens)._type {
            is_array = true;
            // '['
            let token = self.advance(tokens, source)?;
            consume_single_terminal_token!(
                self,
                token,
                TokenType::Symbol(Symbols::LeftBracket),
                TokenType::Symbol(Symbols::LeftBracket),
                source
            );

            self.code_gen
                .push_variable(variable_name)
                .map_err(Self::map_code_gen_err_with_token_line(variable_token.line))?;

            // expression
            self.expression(tokens, source)?;
            self.code_gen.push_op(VM_OPS::ADD);

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

        // vmcode, pop the expression into the variable
        if is_array {
            self.code_gen.pop_temp(0);
            self.code_gen.pop_pointer(1);
            self.code_gen.push_temp();
            self.code_gen.pop_that();
        } else {
            self.code_gen
                .pop_variable(variable_name)
                .map_err(Self::map_code_gen_err_with_token_line(variable_token.line))?;
        }

        self.xml_ast.push("</letStatement>".to_string());
        Ok(())
    }
    fn if_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<ifStatement>".to_string());

        // 'if'
        let token = self.advance(tokens, source)?;
        self.code_gen
            .push_comment(format!("start if, {}", token.line));
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

        // expression is at the top of the stack
        self.expression(tokens, source)?;
        // vmcode
        self.code_gen.push_op(VM_OPS::NOT);
        let l1 = self
            .code_gen
            .get_flow_counter(&self.class_name.clone().unwrap());
        let l2 = self
            .code_gen
            .get_flow_counter(&self.class_name.clone().unwrap());
        self.code_gen.push_if_goto(&l1);

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

        // vmcode
        self.statements(tokens, source)?;
        self.code_gen.push_goto(&l2);
        self.code_gen.push_label(&l1); // Condition did not pass, so we jump to else

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
        // vmcode
        self.code_gen.push_label(&l2);
        self.code_gen
            .push_comment(format!("end if, {}", tokens[self.current - 1].line));

        self.xml_ast.push("</ifStatement>".to_string());
        Ok(())
    }
    fn while_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<whileStatement>".to_string());
        // 'while'
        let token = self.advance(tokens, source)?;
        self.code_gen
            .push_comment(format!("start while, {}", token.line));
        let l1 = self
            .code_gen
            .get_flow_counter(&self.class_name.clone().unwrap());
        let l2 = self
            .code_gen
            .get_flow_counter(&self.class_name.clone().unwrap());
        self.code_gen.push_label(&l1);

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
        self.code_gen.push_op(VM_OPS::NOT);
        self.code_gen.push_if_goto(&l2);

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
        self.code_gen.push_goto(&l1);
        self.code_gen.push_label(&l2);

        // '}'
        let token = self.advance(tokens, source)?;
        consume_single_terminal_token!(
            self,
            token,
            TokenType::Symbol(Symbols::RightBrace),
            TokenType::Symbol(Symbols::RightBrace),
            source
        );
        self.code_gen
            .push_comment(format!("end while, {}", tokens[self.current - 1].line));

        self.xml_ast.push("</whileStatement>".to_string());
        Ok(())
    }
    fn do_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<doStatement>".to_string());

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

        // needs to remove uncessary value from top of stack from function return
        self.code_gen.pop_temp(0);

        self.xml_ast.push("</doStatement>".to_string());
        Ok(())
    }
    fn return_statement(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<returnStatement>".to_string());

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
                self.code_gen.push_integer_constant(0);
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
        self.code_gen.push_return();

        self.xml_ast.push("</returnStatement>".to_string());
        Ok(())
    }

    // Expressions
    fn expression(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        self.xml_ast.push("<expression>".to_string());
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
            let op_token = self.advance(tokens, source)?;
            self.push_terminal(&op_token, source);
            self.term(tokens, source)?;
            // push the op after the term
            match op_token._type {
                TokenType::Symbol(Symbols::Plus) => {
                    self.code_gen.push_op(VM_OPS::ADD);
                }
                TokenType::Symbol(Symbols::Minus) => {
                    self.code_gen.push_op(VM_OPS::SUB);
                }
                TokenType::Symbol(Symbols::And) => {
                    self.code_gen.push_op(VM_OPS::AND);
                }
                TokenType::Symbol(Symbols::Or) => {
                    self.code_gen.push_op(VM_OPS::OR);
                }
                TokenType::Symbol(Symbols::GreaterThan) => {
                    self.code_gen.push_op(VM_OPS::GT);
                }
                TokenType::Symbol(Symbols::LessThan) => {
                    self.code_gen.push_op(VM_OPS::LT);
                }
                TokenType::Symbol(Symbols::Equal) => {
                    self.code_gen.push_op(VM_OPS::EQ);
                }
                // TODO: OS method calls
                TokenType::Symbol(Symbols::Asterisk) => {
                    self.code_gen.call_math_multiply();
                }
                TokenType::Symbol(Symbols::Slash) => {
                    self.code_gen.call_math_divide();
                }
                _ => return Err(String::from("expression codegen, not")),
            }
        }
        self.xml_ast.push("</expression>".to_string());
        Ok(())
    }
    fn subroutine_call(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        // subroutineName | className | varName
        self.identifier(tokens, source)?;
        self.code_gen.push_comment(format!(
            "start subroutine_call, {}",
            tokens[self.current - 1].line
        ));
        let l1_token = &tokens[self.current - 1];
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

                // since this is a call within the class, we need to set this
                self.code_gen.push_pointer(0);
                let no_of_expressions = self.expression_list(tokens, source)?;
                self.code_gen.push_call(
                    &self.class_name.clone().unwrap(),
                    &l1_token.get_source(source),
                    no_of_expressions as i16 + 1,
                );

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
                let l2_token = &tokens[self.current - 1];
                // error means we are trying to call a Class function, push dummy 0 for dummy this arg
                if self
                    .code_gen
                    .push_variable(&l1_token.get_source(source))
                    .is_err()
                {
                    self.code_gen.push_comment(String::from("dummy 0"));
                    self.code_gen.push_integer_constant(0);
                };

                // '('
                let token = self.advance(tokens, source)?;
                consume_single_terminal_token!(
                    self,
                    token,
                    TokenType::Symbol(Symbols::LeftParam),
                    TokenType::Symbol(Symbols::LeftParam),
                    source
                );

                let no_of_expressions = self.expression_list(tokens, source)?;
                self.code_gen
                    .complex_subroutine_call(
                        &l1_token.get_source(source),
                        &l2_token.get_source(source),
                        no_of_expressions as i16 + 1,
                    )
                    .map_err(Self::map_code_gen_err_with_token_line(
                        tokens[self.current - 1].line,
                    ))?;

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
        self.code_gen.push_comment(format!(
            "end subroutine_call, {}",
            tokens[self.current - 1].line
        ));
        Ok(())
    }
    fn term(&mut self, tokens: &[Token], source: &[char]) -> ParserReturn {
        let vm_codegen_err = Err(String::from("Should not, from term codegen path"));

        self.xml_ast.push("<term>".to_string());
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
                // For xml_ast
                self.push_terminal(&token, source);
                self.advance(tokens, source)?;
                // For vm codegen
                match token._type {
                    TokenType::Keyword(
                        ReservedKeywords::True | ReservedKeywords::False | ReservedKeywords::Null,
                    )
                    | TokenType::String => {
                        // TODO
                    }
                    TokenType::Keyword(ReservedKeywords::This) => {
                        self.code_gen.push_pointer(0);
                    }
                    TokenType::Integer(x) => {
                        self.code_gen.push_integer_constant(x as i16);
                    }
                    _ => {
                        return vm_codegen_err;
                    }
                }
            }
            TokenType::Identifier => {
                // Decide between varName, varName '[' and subroutine_call
                // LL(2)
                match self.peek_n(1, tokens) {
                    Some(next_token) => {
                        match next_token._type {
                            // index on variable
                            TokenType::Symbol(Symbols::LeftBracket) => {
                                self.identifier(tokens, source)?;
                                let variable_token = &tokens[self.current - 1];
                                let variable_name = &variable_token.get_source(source);
                                self.code_gen.push_variable(variable_name).map_err(
                                    Self::map_code_gen_err_with_token_line(variable_token.line),
                                )?;

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

                                self.code_gen.push_op(VM_OPS::ADD);
                                self.code_gen.pop_pointer(1); // set array base address + idx
                                self.code_gen.push_that();
                            }
                            TokenType::Symbol(Symbols::LeftBrace | Symbols::Dot) => {
                                self.subroutine_call(tokens, source)?;
                            }
                            _ => {
                                // Just varName
                                self.identifier(tokens, source)?;
                                let variable_token = &tokens[self.current - 1];
                                let variable_name = &variable_token.get_source(source);
                                self.code_gen.push_variable(variable_name).map_err(
                                    Self::map_code_gen_err_with_token_line(variable_token.line),
                                )?;
                            }
                        }
                    }
                    None => {
                        // Just varName, should almost never encounter this
                        #[cfg(feature = "debug")]
                        {
                            println!("term, varname with no L(2) peek")
                        }
                        self.identifier(tokens, source)?;
                        let variable_token = &tokens[self.current - 1];
                        let variable_name = &variable_token.get_source(source);
                        self.code_gen
                            .push_variable(variable_name)
                            .map_err(Self::map_code_gen_err_with_token_line(variable_token.line))?;
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

                let op = match token._type {
                    TokenType::Symbol(Symbols::Minus) => VM_OPS::NEG,
                    TokenType::Symbol(Symbols::Tilde) => VM_OPS::NOT,
                    _ => return vm_codegen_err,
                };
                self.term(tokens, source)?;
                self.code_gen.push_op(op);
            }
            _ => {
                return Err(format!(
                    "Expected a term, got {} on line {}",
                    token.get_source(source),
                    token.line
                ));
            }
        }
        self.xml_ast.push("</term>".to_string());
        Ok(())
    }
    // Needs to return the number of expressions it pushed onto the stack, for vm codegen
    fn expression_list(&mut self, tokens: &[Token], source: &[char]) -> Result<usize, String> {
        self.xml_ast.push("<expressionList>".to_string());
        let mut no_of_expressions = 0;
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
                no_of_expressions += 1;

                // (',' expression)*
                while let TokenType::Symbol(Symbols::Comma) = self.peek(tokens)._type {
                    // ','
                    let token = self.advance(tokens, source)?;
                    self.push_terminal(&token, source);

                    self.expression(tokens, source)?;
                    no_of_expressions += 1;
                }
            }
            _ => {}
        }
        self.xml_ast.push("</expressionList>".to_string());
        Ok(no_of_expressions)
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
    fn peek_n(&self, n: usize, tokens: &[Token]) -> Option<Token> {
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
    fn map_code_gen_err_with_token_line(token_line: usize) -> impl Fn(String) -> String {
        move |e: String| -> String { format!("VM codegen error: {}, on line {}", e, token_line) }
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
        println!("{:?}", parser.xml_ast);
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
        assert_eq!(parser.xml_ast.len(), 3);
        assert_eq!(parser.xml_ast[0], "<keyword>");
        assert_eq!(parser.xml_ast[1], "let");
        assert_eq!(parser.xml_ast[2], "</keyword>");

        let source = "\"funny string\"".chars().collect::<Vec<char>>();
        let token = Token {
            _type: TokenType::String,
            start: 0,
            length: 14,
            line: 1,
        };
        let mut parser = Parser::new();
        parser.push_terminal(&token, &source);
        assert_eq!(parser.xml_ast.len(), 3);
        assert_eq!(parser.xml_ast[0], "<stringConstant>");
        assert_eq!(parser.xml_ast[1], "funny string");
        assert_eq!(parser.xml_ast[2], "</stringConstant>");
    }
}
