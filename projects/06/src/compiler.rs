use std::thread::current;

use crate::{Assembler, Scanner, SymbolTable, Token, TokenType};

#[derive(Default)]
pub struct Compiler {
    rom: Vec<[char; 16]>, // Final vector of all the 16 bit ROMs instructions
    tokens: Vec<Token>,
    symbol_table: SymbolTable,
    had_error: bool,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }
    fn compile(&mut self, source: String) {
        let source = source.chars().collect::<Vec<char>>();
        self.consume_source(&source);
        self.first_pass();
        self.second_pass(&source);
        self.hack(&source);
    }

    fn first_pass(&mut self) {
        let mut new_tokens: Vec<Token> = Vec::new();
        let mut line_tokens: Vec<Token> = Vec::new();
        for token in &self.tokens {
            line_tokens.push(token.clone());
            if let Token::NormalToken {
                _type,
                start: _,
                length: _,
                line: _,
            } = token
            {
                match _type {
                    TokenType::NewLine => {
                        // Remove empty lines
                        if line_tokens.len() == 1 {
                            line_tokens = Vec::new();
                        } else {
                            new_tokens.append(&mut line_tokens);
                        }
                    }
                    TokenType::EOF => {
                        new_tokens.append(&mut line_tokens);
                    }
                    _ => {
                        // Continue
                    }
                }
            }
        }
        self.tokens = new_tokens;
    }
    fn second_pass(&mut self, source: &Vec<char>) {
        // Capture all the used memory and instruction labels
        let mut current_line = 0;
        let mut new_tokens: Vec<Token> = Vec::new();
        let mut line_tokens: Vec<Token> = Vec::new();
        for token in self.tokens.clone() {
            line_tokens.push(token.clone());
            if let Token::NormalToken {
                _type,
                start: _,
                length: _,
                line: _,
            } = &token
            {
                match _type {
                    TokenType::NewLine | TokenType::EOF => {
                        if let Token::NormalToken {
                            _type,
                            start: _,
                            length: _,
                            line: _,
                        } = &line_tokens[0]
                        {
                            match _type {
                                TokenType::LeftParam => {
                                    self.instruction_label(source, &line_tokens, current_line);
                                    line_tokens = Vec::new();
                                }
                                TokenType::At => {
                                    self.potential_memory_label(source, &line_tokens);
                                    new_tokens.append(&mut line_tokens);
                                    current_line += 1;
                                }
                                _ => {
                                    new_tokens.append(&mut line_tokens);
                                    current_line += 1;
                                }
                            }
                        }
                    }
                    _ => {
                        // Continue
                    }
                }
            }
        }
        self.tokens = new_tokens;
    }

    fn instruction_label(
        &mut self,
        source: &Vec<char>,
        line_tokens: &Vec<Token>,
        current_line: usize,
    ) {
        // When passing control to this function, left paran and last EOL is already taken, we just need to parse the rest
        if line_tokens.len() != 4 {
            return self.error(
                source,
                line_tokens[0].clone(),
                String::from("Expected instruction label"),
            );
        }
        if let Token::NormalToken {
            _type: TokenType::RightParam,
            start: _,
            length: _,
            line: _,
        } = line_tokens[2]
        {
            // Continue
        } else {
            return self.error(
                source,
                line_tokens[0].clone(),
                String::from("Expected instruction label"),
            );
        }
        if let Token::NormalToken {
            _type: TokenType::Label,
            start: _,
            length: _,
            line: _,
        } = line_tokens[1]
        {
            // Add the label
            self.symbol_table
                .insert_instruction_label(source, &line_tokens[1], current_line);
        } else {
            self.error(
                source,
                line_tokens[0].clone(),
                String::from("Expected instruction label"),
            );
        }
    }
    fn potential_memory_label(&mut self, source: &Vec<char>, line_tokens: &Vec<Token>) {
        // When passing control to this function, we already consumed the first [TokenType::At]
        // If there is an error with the memory label, we deal with it on the compilation step
        if line_tokens.len() != 3 {
            return self.error(
                source,
                line_tokens[0].clone(),
                String::from("Expected memory label"),
            );
        }

        if let Token::NormalToken {
            _type: TokenType::Label,
            start: _,
            length: _,
            line: _,
        } = line_tokens[1]
        {
            self.symbol_table
                .insert_memory_label(source, &line_tokens[1]);
        }
        // Could still be a valid token, we only care about processing memory labels here
    }

    /// Play on the hack langauge eh
    fn hack(&mut self, source: &Vec<char>) {
        let mut line_tokens: Vec<Token> = Vec::new();
        for token in self.tokens.clone() {
            line_tokens.push(token.clone());
            if let Token::NormalToken {
                _type,
                start: _,
                length: _,
                line: _,
            } = &token
            {
                match _type {
                    TokenType::NewLine | TokenType::EOF => {
                        if let Token::NormalToken {
                            _type,
                            start: _,
                            length: _,
                            line: _,
                        } = &line_tokens[0]
                        {
                            match _type {
                                TokenType::At => {
                                    // A-instruction
                                    self.a_instruction(source, &line_tokens);
                                }
                                _ => {
                                    // Potential C-instruction
                                }
                            }
                        }
                    }
                    _ => {
                        // Continue
                    }
                }
            }
        }
    }

    fn a_instruction(&mut self, source: &Vec<char>, line_tokens: &Vec<Token>) {
        if self.tokens.len() != 3 {
            // Not a valid a_instruction, disregard
            self.error(
                source,
                self.tokens[0].clone(),
                String::from("Invalid A-Instruction"),
            );
        }

        match self.symbol_table.get_symbol(source, &line_tokens[1]) {
            Ok(value) => match Assembler::a_instruction(value) {
                Err(msg) => self.error(source, line_tokens[1].clone(), msg),
                Ok(rom_instruction) => self.rom.push(rom_instruction),
            },
            Err(msg) => self.error(source, line_tokens[1].clone(), msg),
        }
        // Error already handled and will be dealt later on
    }
    fn c_instruction(&mut self, source: &Vec<char>, line_tokens: &Vec<Token>) {
        let mut contains_equal = false;
        let mut contains_semi = false;
        for token in line_tokens {
            if let Token::NormalToken {
                _type,
                start: _,
                length: _,
                line: _,
            } = token
            {
                match _type {
                    TokenType::SemiColon => {
                        if !contains_semi {
                            contains_semi = true
                        } else {
                            return self.error(
                                source,
                                token.clone(),
                                String::from("Expected only 1 semicolons"),
                            );
                        }
                    }
                    TokenType::Equal => {
                        if !contains_equal {
                            contains_equal = true;
                        } else {
                            return self.error(
                                source,
                                token.clone(),
                                String::from("Expected only 1 equals"),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut current_tokens: Vec<Token> = Vec::new();
        for token in line_tokens {
            current_tokens.push(token.clone());
            if let Token::NormalToken {
                _type,
                start: _,
                length: _,
                line: _,
            } = token
            {
                if contains_equal {}
            }
        }
    }

    /// Consumes source to generate all tokens
    fn consume_source(&mut self, source: &Vec<char>) {
        let mut scanner = Scanner::new();
        loop {
            let token = scanner.scan_token(source);
            let finished = match &token {
                Token::NormalToken {
                    _type,
                    start: _,
                    length: _,
                    line: _,
                } => {
                    matches!(_type, TokenType::EOF)
                }
                Token::ErrorToken { line: _, msg } => {
                    self.error(source, token.clone(), msg.clone());
                    false
                }
            };
            self.tokens.push(token);
            if finished {
                break;
            }
        }
    }

    fn error(&mut self, source: &Vec<char>, token: Token, msg: String) {
        // surpress all other errors except the first one
        if self.had_error {
            return;
        } else {
            self.had_error = true;
        }

        match token {
            Token::NormalToken {
                _type,
                start,
                length,
                line,
            } => {
                println!(
                    "Line {} Error, at {}: {}",
                    line,
                    source[start..start + length].iter().collect::<String>(),
                    msg
                )
            }
            Token::ErrorToken { line, msg: _ } => {
                println!("Line {}, Error: {}", line, msg)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_pass_skip_empty_lines() {
        let source = "\n\n\n\n@";
        let source = source.chars().collect::<Vec<char>>();
        let mut compiler = Compiler::new();
        compiler.consume_source(&source);
        compiler.first_pass();
        assert_eq!(compiler.tokens.len(), 2);

        let source = "\n\n@10\n\nM=A";
        let source = source.chars().collect::<Vec<char>>();
        let mut compiler = Compiler::new();
        compiler.consume_source(&source);
        compiler.first_pass();
        assert_eq!(compiler.tokens.len(), 7);
    }
    #[test]
    fn test_second_pass_instruction_label() {
        let source = "@r1\n(LOOP)\n";
        let source = source.chars().collect::<Vec<char>>();
        let mut compiler = Compiler::new();
        compiler.consume_source(&source);
        compiler.first_pass();
        compiler.second_pass(&source);
        assert_eq!(compiler.tokens.len(), 4);
    }
}
