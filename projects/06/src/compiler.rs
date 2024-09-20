use std::collections::HashMap;

use crate::{scanner::Scanner, Token, TokenType};

struct SymbolTable {
    table: HashMap<String, usize>,
    current_memory: usize,
}
impl SymbolTable {
    fn new() -> Self {
        Self::default()
    }

    fn get_label(&self, source: &Vec<char>, token: &Token) -> String {
        if let Token::NormalToken {
            _type,
            start,
            length,
            line: _,
        } = token
        {
            source[start.to_owned()..(start.to_owned() + length.to_owned())]
                .iter()
                .collect::<String>()
        } else {
            // Should not be possible, as only valid tokens should be passed
            panic!("Should be normal Token")
        }
    }

    fn insert_instruction_label(
        &mut self,
        source: &Vec<char>,
        token: &Token,
        instruction_location: usize,
    ) {
        self.table
            .insert(self.get_label(source, token), instruction_location);
    }
    fn insert_memory_label(&mut self, source: &Vec<char>, token: &Token) {
        self.table
            .insert(self.get_label(source, token), self.current_memory);
        self.current_memory += 1;
    }

    fn get_symbol(&self, source: &Vec<char>, token: &Token) -> usize {
        if let Token::NormalToken {
            _type,
            start: _,
            length: _,
            line: _,
        } = token
        {
            match _type {
                TokenType::SP => 0,
                TokenType::LCL => 1,
                TokenType::ARG => 2,
                TokenType::THIS => 3,
                TokenType::THAT => 4,
                TokenType::R0 => 0,
                TokenType::R1 => 1,
                TokenType::R2 => 2,
                TokenType::R3 => 3,
                TokenType::R4 => 4,
                TokenType::R5 => 5,
                TokenType::R6 => 6,
                TokenType::R7 => 7,
                TokenType::R8 => 8,
                TokenType::R9 => 9,
                TokenType::R10 => 10,
                TokenType::R11 => 11,
                TokenType::R12 => 12,
                TokenType::R13 => 13,
                TokenType::R14 => 14,
                TokenType::R15 => 15,
                TokenType::SCREEN => 16384,
                TokenType::KBD => 24576,
                TokenType::Label => self
                    .table
                    .get(&self.get_label(source, token))
                    .unwrap()
                    .to_owned(),
                _ => {
                    // Should not happen either
                    panic!("Should be a label token");
                }
            }
        } else {
            panic!("Should be normal Token")
        }
    }
}
impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            table: HashMap::default(),
            current_memory: 16,
        }
    }
}

#[derive(Default)]
pub struct Compiler {
    rom: Vec<[char; 16]>, // Final vector of all the 16 bit ROMs instructions
    tokens: Vec<Token>,
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

    // fn instruction(&self, source: &Vec<char>) {
    //     // If this fails, something went wrong
    //     assert!(self.tokens.len() > 0);

    //     if let Token::NormalToken {
    //         _type,
    //         start,
    //         length,
    //         line,
    //     } = self.tokens[0]
    //     {
    //         match _type {
    //             TokenType::NewLine => {
    //                 // Should just skip, empty newline
    //                 return;
    //             }
    //             TokenType::EOF => {
    //                 // IDK
    //                 return;
    //             }
    //             TokenType::At => {
    //                 return self.a_instruction(source);
    //             }
    //         }
    //     } else {
    //         // TODO: ErrorToken handling
    //         return;
    //     }
    // }
    // fn a_instruction(&mut self, source: &Vec<char>) {
    //     if self.tokens.len() != 2 {
    //         // Not a valid a_instruction, disregard
    //         self.error(
    //             source,
    //             self.tokens[0].clone(),
    //             String::from("Invalid A-Instruction"),
    //         );
    //     }
    // }

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
        let mut compiler = Compiler::new();
        compiler.compile(source.to_string());
        assert_eq!(compiler.tokens.len(), 2);

        let source = "\n\n@10\n\nM=A";
        let mut compiler = Compiler::new();
        compiler.compile(source.to_string());
        assert_eq!(compiler.tokens.len(), 7);
    }
}
