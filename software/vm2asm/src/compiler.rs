use crate::{code_gen::CodeGen, LineSource, Parser};

pub struct Compiler {
    line_sources: Vec<LineSource>,
    asm: Vec<String>, // Output of compiled asm
    had_error: bool,
}

impl Compiler {
    fn new(source: String) -> Self {
        let line_sources = Parser::parse(source);
        Self {
            asm: Vec::default(),
            line_sources,
            // symbol_table: SimpleSymbolTable::new(),
            had_error: false,
        }
    }

    pub fn compile(source: String) -> Option<Vec<String>> {
        let mut compiler = Self::new(source);
        Some(Vec::default())
    }

    pub fn run(&mut self) {
        for line_source in self.line_sources.clone() {
            let tokens = &line_source.tokens;
            match tokens.len() {
                1 => self.single_command(&line_source),
                _ => self.error(
                    line_source.line,
                    String::from("Unknown token length for command"),
                ),
            }
        }
    }

    fn single_command(&mut self, line_source: &LineSource) {
        assert!(line_source.tokens.len() == 1);
        let command = &line_source.tokens[0];
        match command.as_str() {
            "add" => {
                self.asm.append(&mut CodeGen::add());
            }
            "sub" => {}
            "neg" => {}
            "eq" => {}
            "gt" => {}
            "lt" => {}
            "and" => {}
            "or" => {}
            "not" => {}
            "return" => self.error(line_source.line, String::from("Not Implemented yet")), // Not implemented yet
            _ => self.error(line_source.line, format!("Unknown command, {}", command)),
        }
    }

    fn error(&mut self, line: usize, msg: String) {
        // We have already encountered the first error
        // So ignore future errors
        if self.had_error {
            return;
        } else {
            self.had_error = true;
        }
        println!("Error on line {}: {}", line + 1, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom() {
        let source = String::from("add");
        let mut compiler = Compiler::new(source);
        compiler.run();
        assert_eq!(compiler.asm[0], String::from("//add"));
    }
}
