use std::collections::HashMap;

use crate::{Parser, ParserReturn, ReservedKeywords, Token, TokenType};

enum VariableKind {
    Field,
    Static,
    Argument,
    Local,
}

enum VariableType {
    Int,
    Char,
    Boolean,
    Identifier(String),
}

// type
// kind
// no.
struct VariableMetaData {
    _type: VariableType,
    kind: VariableKind,
    number: usize,
}

pub struct CodeGen {
    class_symbol_table: HashMap<String, VariableMetaData>,
    subroutine_symbol_table: HashMap<String, VariableMetaData>,
    field_counter: usize,
    static_counter: usize,
    argument_counter: usize,
    local_counter: usize,
}
impl CodeGen {
    fn new() -> Self {
        Self::default()
    }
    fn insert_class_variable(
        &mut self,
        name: String,
        kind: Token,
        _type: Token,
        source: &[char],
    ) -> ParserReturn {
        let variable_kind = match &kind._type {
            TokenType::Keyword(ReservedKeywords::Static) => VariableKind::Static,
            TokenType::Keyword(ReservedKeywords::Field) => VariableKind::Field,
            _ => {
                return Err(Parser::error_expected_token_type(
                    &kind,
                    &[
                        TokenType::Keyword(ReservedKeywords::Static),
                        TokenType::Keyword(ReservedKeywords::Field),
                    ],
                    source,
                ))
            }
        };
        let variable_type = match &_type._type {
            TokenType::Keyword(ReservedKeywords::Int) => VariableType::Int,
            TokenType::Keyword(ReservedKeywords::Char) => VariableType::Char,
            TokenType::Keyword(ReservedKeywords::Boolean) => VariableType::Boolean,
            TokenType::Identifier => VariableType::Identifier(_type.get_source(source)),
            _ => {
                return Err(Parser::error_expected_token_type(
                    &_type,
                    &[
                        TokenType::Keyword(ReservedKeywords::Int),
                        TokenType::Keyword(ReservedKeywords::Char),
                        TokenType::Keyword(ReservedKeywords::Boolean),
                        TokenType::Identifier,
                    ],
                    source,
                ))
            }
        };
        let number = match variable_kind {
            VariableKind::Static => {
                self.static_counter += 1;
                self.static_counter - 1
            }
            VariableKind::Field => {
                self.field_counter += 1;
                self.field_counter - 1
            }
            _ => panic!("Should not fail insert class variable"),
        };
        match self.class_symbol_table.insert(
            name.clone(),
            VariableMetaData {
                _type: variable_type,
                kind: variable_kind,
                number,
            },
        ) {
            Some(_) => Err(format!(
                "Class variable {} is defined again on line {}",
                name, kind.line
            )),
            None => Ok(()),
        }
    }
    fn reset_subroutine_table(&mut self, class_name: String) {
        self.subroutine_symbol_table = HashMap::default();
        self.local_counter = 0;
        self.argument_counter = 0;
    }
    fn insert_subroutine_variable(
        &mut self,
        name: String,
        kind: Token,
        _type: Token,
        source: &[char],
    ) -> ParserReturn {
        let variable_kind = match &kind._type {
            TokenType::Keyword(ReservedKeywords::Static) => VariableKind::Static,
            TokenType::Keyword(ReservedKeywords::Field) => VariableKind::Field,
            _ => {
                return Err(Parser::error_expected_token_type(
                    &kind,
                    &[
                        TokenType::Keyword(ReservedKeywords::Static),
                        TokenType::Keyword(ReservedKeywords::Field),
                    ],
                    source,
                ))
            }
        };
        let variable_type = match &_type._type {
            TokenType::Keyword(ReservedKeywords::Int) => VariableType::Int,
            TokenType::Keyword(ReservedKeywords::Char) => VariableType::Char,
            TokenType::Keyword(ReservedKeywords::Boolean) => VariableType::Boolean,
            TokenType::Identifier => VariableType::Identifier(_type.get_source(source)),
            _ => {
                return Err(Parser::error_expected_token_type(
                    &_type,
                    &[
                        TokenType::Keyword(ReservedKeywords::Int),
                        TokenType::Keyword(ReservedKeywords::Char),
                        TokenType::Keyword(ReservedKeywords::Boolean),
                        TokenType::Identifier,
                    ],
                    source,
                ))
            }
        };
        let number = match variable_kind {
            VariableKind::Static => {
                self.static_counter += 1;
                self.static_counter - 1
            }
            VariableKind::Field => {
                self.field_counter += 1;
                self.field_counter - 1
            }
            _ => panic!("Should not fail insert class variable"),
        };
        match self.class_symbol_table.insert(
            name.clone(),
            VariableMetaData {
                _type: variable_type,
                kind: variable_kind,
                number,
            },
        ) {
            Some(_) => Err(format!(
                "Class variable {} is defined again on line {}",
                name, kind.line
            )),
            None => Ok(()),
        }
    }
}
impl Default for CodeGen {
    fn default() -> Self {
        Self {
            class_symbol_table: HashMap::default(),
            subroutine_symbol_table: HashMap::default(),
            field_counter: 0,
            static_counter: 0,
            argument_counter: 0,
            local_counter: 0,
        }
    }
}
