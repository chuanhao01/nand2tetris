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
        // self.class_symbol_table.insert(name, VariableMetaData{_type, kind, })
        Ok(())
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
