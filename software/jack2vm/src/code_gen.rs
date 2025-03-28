use std::collections::HashMap;

use crate::{Parser, ParserReturn, ReservedKeywords, Token, TokenType, VM_OPS};

#[derive(Debug)]
pub enum VariableKind {
    Field,
    Static,
    Argument,
    Local,
}
impl VariableKind {
    pub fn to_vm_segment(&self) -> String {
        let segment = match self {
            // TODO
            Self::Field => "this",
            Self::Static => "static",
            Self::Argument => "argument",
            Self::Local => "local",
        };
        segment.to_string()
    }
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

type CodeGenResult = Result<(), String>;

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
    flow_counter: usize,
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
    pub fn push_pointer(&mut self, no: i16) {
        self.vm_code.push(format!("push pointer {}", no))
    }
    pub fn pop_pointer(&mut self, no: i16) {
        self.vm_code.push(format!("pop pointer {}", no))
    }
    pub fn push_op(&mut self, op: VM_OPS) {
        self.vm_code.push(op.to_vm_string());
    }
    // handling variables
    fn get_variable_metadata(&self, variable_name: &String) -> Result<&VariableMetaData, String> {
        if let Some(variable) = self.subroutine_symbol_table.get(variable_name) {
            Ok(variable)
        } else if let Some(variable) = self.class_symbol_table.get(variable_name) {
            Ok(variable)
        } else {
            return Err(format!("Variable {} not declared before", variable_name));
        }
    }
    pub fn push_variable(&mut self, variable_name: &String) -> CodeGenResult {
        let variable_metadata = self.get_variable_metadata(variable_name)?;
        self.vm_code.push(format!(
            "push {} {}",
            variable_metadata.kind.to_vm_segment(),
            variable_metadata.number
        ));
        Ok(())
    }
    pub fn pop_variable(&mut self, variable_name: &String) -> CodeGenResult {
        let variable_metadata = self.get_variable_metadata(variable_name)?;
        self.vm_code.push(format!(
            "pop {} {}",
            variable_metadata.kind.to_vm_segment(),
            variable_metadata.number
        ));
        Ok(())
    }
    pub fn pop_temp(&mut self, idx: i16) {
        self.vm_code.push(format!("pop temp {}", idx));
    }
    pub fn push_temp(&mut self) {
        self.vm_code.push(String::from("push temp 0"));
    }
    fn push_temp_idx(&mut self, idx: i16) {
        self.vm_code.push(format!("push temp {}", idx));
    }

    //. Only used for arrays, that 0
    pub fn push_that(&mut self) {
        self.vm_code.push(String::from("push that 0"));
    }
    pub fn pop_that(&mut self) {
        self.vm_code.push(String::from("pop that 0"));
    }

    // Handling if-goto, goto and labels
    pub fn get_flow_counter(&mut self, class_name: &str) -> String {
        self.flow_counter += 1;
        format!("{}.flow.{}", class_name, self.flow_counter - 1)
    }
    pub fn push_if_goto(&mut self, label: &str) {
        self.vm_code.push(format!("if-goto {}", label));
    }
    pub fn push_goto(&mut self, label: &str) {
        self.vm_code.push(format!("goto {}", label));
    }
    pub fn push_label(&mut self, label: &str) {
        self.vm_code.push(format!("label {}", label));
    }
    // functions
    pub fn push_function(&mut self, class_name: &str, function_name: &str) {
        let n_locals = self
            .subroutine_symbol_table
            .iter()
            .filter(|(_, v)| matches!(v.kind, VariableKind::Local))
            .count();
        self.vm_code.push(format!(
            "function {}.{} {}",
            class_name, function_name, n_locals
        ))
    }
    pub fn constructor_alloc(&mut self) {
        let n_fields = self
            .class_symbol_table
            .iter()
            .filter(|(_, v)| matches!(v.kind, VariableKind::Field))
            .count();
        self.push_integer_constant(n_fields as i16);
        self.vm_code.push(String::from("call Memory.alloc 1"));
        self.pop_pointer(0);
    }
    pub fn push_call(&mut self, class_name: &str, function_name: &str, n_args: i16) {
        self.vm_code
            .push(format!("call {}.{} {}", class_name, function_name, n_args));
    }
    pub fn complex_subroutine_call(&mut self, l1: &str, l2: &str, n_args: i16) -> CodeGenResult {
        let variable_meta_data = if let Some(variable) = self.subroutine_symbol_table.get(l1) {
            variable
        } else if let Some(variable) = self.class_symbol_table.get(l1) {
            variable
        } else {
            // is a Class function call
            self.vm_code.push(format!("call {}.{} {}", l1, l2, n_args));
            return Ok(());
        };
        match &variable_meta_data._type {
            VariableType::Identifier(class_name) => {
                self.vm_code
                    .push(format!("call {}.{} {}", class_name, l2, n_args));
            }
            _ => {
                return Err(format!(
                    "subroutine call on, {}, with type {:?} is not possible",
                    l1, variable_meta_data._type
                ))
            }
        }
        Ok(())
    }
    pub fn push_return(&mut self) {
        self.vm_code.push(String::from("return"));
    }
    pub fn push_comment(&mut self, comment: String) {
        #[cfg(feature = "debug")]
        {
            self.vm_code.push(format!("// {}", comment))
        }
    }
    // For easier Math syntax
    /// For Math.multiply and Math.divide
    fn setup_math_call(&mut self) {
        self.pop_temp(0);
        self.pop_temp(1);
        self.push_integer_constant(0);
        self.push_temp_idx(0);
        self.push_temp_idx(1);
    }
    /// Called when encountering * in the parser, after both terms are already added to the vm_code
    pub fn call_math_multiply(&mut self) {
        self.setup_math_call();
        self.push_call("Math", "multiply", 3);
    }
    pub fn call_math_divide(&mut self) {
        self.setup_math_call();
        self.push_call("Math", "divide", 3);
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
            flow_counter: 0,
        }
    }
}
