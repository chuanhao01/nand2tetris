use crate::{Token, TokenType};
use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, usize>,
    current_memory: usize,
}
impl SymbolTable {
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

    pub fn insert_instruction_label(
        &mut self,
        source: &Vec<char>,
        token: &Token,
        instruction_location: usize,
    ) {
        self.table
            .insert(self.get_label(source, token), instruction_location);
    }
    pub fn insert_memory_label(&mut self, source: &Vec<char>, token: &Token) {
        let label = self.get_label(source, token);
        if self.table.contains_key(&label) {
            return;
        }
        self.table.insert(label, self.current_memory);
        self.current_memory += 1;
    }

    pub fn get_symbol(&self, source: &Vec<char>, token: &Token) -> Result<usize, String> {
        if let Token::NormalToken {
            _type,
            start: _,
            length: _,
            line: _,
        } = token
        {
            match _type {
                TokenType::SP => Ok(0),
                TokenType::LCL => Ok(1),
                TokenType::ARG => Ok(2),
                TokenType::THIS => Ok(3),
                TokenType::THAT => Ok(4),
                TokenType::R0 => Ok(0),
                TokenType::R1 => Ok(1),
                TokenType::R2 => Ok(2),
                TokenType::R3 => Ok(3),
                TokenType::R4 => Ok(4),
                TokenType::R5 => Ok(5),
                TokenType::R6 => Ok(6),
                TokenType::R7 => Ok(7),
                TokenType::R8 => Ok(8),
                TokenType::R9 => Ok(9),
                TokenType::R10 => Ok(10),
                TokenType::R11 => Ok(11),
                TokenType::R12 => Ok(12),
                TokenType::R13 => Ok(13),
                TokenType::R14 => Ok(14),
                TokenType::R15 => Ok(15),
                TokenType::SCREEN => Ok(16384),
                TokenType::KBD => Ok(24576),
                TokenType::Label => match self.table.get(&self.get_label(source, token)) {
                    Some(value) => Ok(value.to_owned()),
                    None => Err(String::from("Expected symbol")),
                },
                _ => {
                    // Should not happen either
                    Err(String::from("Unexpected Token"))
                }
            }
        } else {
            Err(String::from("Error Token"))
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
