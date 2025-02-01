use std::collections::HashMap;

use crate::{Parser, ParserReturn, ReservedKeywords, Token, TokenType, VM_OPS};

#[derive(Debug)]
pub enum VariableKind {
    Field,
    Static,
    Argument,
    Local,
}

#[derive(Debug)]
pub enum VariableType {
    Int,
    Char,
    Boolean,
    Identifier(String),
}

// type
// kind
// no.
#[derive(Debug)]
pub struct VariableMetaData {
    _type: VariableType,
    kind: VariableKind,
    number: usize,
}

/// Used by the Parser
/// subroutine symbol table is reset by the subroutine dec before used
pub struct CodeGen {
    pub class_symbol_table: HashMap<String, VariableMetaData>,
    pub subroutine_symbol_table: HashMap<String, VariableMetaData>,
    vm_code: Vec<String>,
    field_counter: usize,
    static_counter: usize,
    argument_counter: usize,
    local_counter: usize,
}
impl CodeGen {
    fn new() -> Self {
        Self::default()
    }
    pub fn gen_vm_code(&self) -> String {
        self.vm_code.join("\n")
    }
    pub fn insert_class_variable(
        &mut self,
        name: String,
        kind: &Token,
        _type: &Token,
        source: &[char],
    ) -> ParserReturn {
        let variable_kind = match kind._type {
            TokenType::Keyword(ReservedKeywords::Static) => VariableKind::Static,
            TokenType::Keyword(ReservedKeywords::Field) => VariableKind::Field,
            _ => {
                return Err(Parser::error_expected_token_type(
                    kind,
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
                    _type,
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
    pub fn reset_subroutine_table(&mut self, class_name: String) {
        self.subroutine_symbol_table = HashMap::new();
        self.subroutine_symbol_table.insert(
            String::from("this"),
            VariableMetaData {
                _type: VariableType::Identifier(class_name),
                number: 0,
                kind: VariableKind::Argument,
            },
        );
        self.argument_counter = 1; // Because of this
        self.local_counter = 0;
    }
    pub fn insert_subroutine_variable(
        &mut self,
        name: String,
        variable_kind: VariableKind,
        _type: &Token,
        source: &[char],
    ) -> ParserReturn {
        if let Some(existing_variable_meta_data) = self.class_symbol_table.get(&name) {
            return Err(format!("Variable with the same name declared again at line {}, was declared previously as a {:?} variable", _type.line, existing_variable_meta_data.kind));
        }
        match variable_kind {
            VariableKind::Local | VariableKind::Argument => {}
            _ => {
                panic!("Wow, the person that coded this should not have done this. insert subroutine variable failed on kind")
            }
        }
        let variable_type = match _type._type {
            TokenType::Keyword(ReservedKeywords::Int) => VariableType::Int,
            TokenType::Keyword(ReservedKeywords::Char) => VariableType::Char,
            TokenType::Keyword(ReservedKeywords::Boolean) => VariableType::Boolean,
            TokenType::Identifier => VariableType::Identifier(_type.get_source(source)),
            _ => {
                return Err(Parser::error_expected_token_type(
                    _type,
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
            VariableKind::Argument => {
                self.argument_counter += 1;
                self.argument_counter - 1
            }
            VariableKind::Local => {
                self.local_counter += 1;
                self.local_counter - 1
            }
            _ => panic!("Should not fail insert class variable"),
        };
        match self.subroutine_symbol_table.insert(
            name.clone(),
            VariableMetaData {
                _type: variable_type,
                kind: variable_kind,
                number,
            },
        ) {
            // Discrepency with insert_class_variable since we don't pass in a kind token
            // We use _type instead of kind.line
            Some(_) => Err(format!(
                "Class variable {} is defined again on line {}",
                name, _type.line
            )),
            None => Ok(()),
        }
    }
    // handling expressions
    pub fn push_integer_constant(&mut self, x: i16) {
        self.vm_code.push(format!("push constant {}", x));
    }
    pub fn push_op(&mut self, op: VM_OPS) {
        self.vm_code.push(op.to_vm_string());
    }
}
#[allow(clippy::derivable_impls)]
impl Default for CodeGen {
    fn default() -> Self {
        Self {
            class_symbol_table: HashMap::default(),
            subroutine_symbol_table: HashMap::default(),
            vm_code: Vec::default(),
            field_counter: 0,
            static_counter: 0,
            argument_counter: 0,
            local_counter: 0,
        }
    }
}
