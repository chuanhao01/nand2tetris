use crate::{CodeGen, LineSource, MemorySegments, Parser};

pub struct Compiler {
    line_sources: Vec<LineSource>,
    asm: Vec<String>, // Output of compiled asm
    file_name: String,
    had_error: bool,
}

impl Compiler {
    fn new(source: String, file_name: String) -> Self {
        let line_sources = Parser::parse(source);
        Self {
            asm: Vec::default(),
            line_sources,
            file_name,
            had_error: false,
        }
    }

    pub fn compile(source: String, file_name: String) -> Option<Vec<String>> {
        let mut compiler = Self::new(source, file_name);
        compiler.run();
        if compiler.had_error {
            None
        } else {
            Some(compiler.asm)
        }
    }

    pub fn run(&mut self) {
        for line_source in self.line_sources.clone() {
            let tokens = &line_source.tokens;
            match tokens.len() {
                1 => self.single_command(&line_source),
                3 => self.triple_command(&line_source),
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
            "add" => self.asm.append(&mut CodeGen::add()),
            "sub" => self.asm.append(&mut CodeGen::sub()),
            "neg" => self.asm.append(&mut CodeGen::neg()),
            "eq" => {}
            "gt" => {}
            "lt" => {}
            "and" => {}
            "or" => {}
            "not" => {}
            "return" => self.error(line_source.line, String::from("Not Implemented yet")), // Not implemented yet
            _ => self.error(
                line_source.line,
                format!("Unknown single command, {}", command),
            ),
        }
    }
    fn triple_command(&mut self, line_source: &LineSource) {
        assert!(line_source.tokens.len() == 3);
        let command = &line_source.tokens[0];
        match command.as_str() {
            "pop" => self.pop_segment(line_source),
            "push" => self.push_segment(line_source),
            _ => self.error(
                line_source.line,
                format!("Unknown triple command, starting from {}", command),
            ),
        }
    }
    fn pop_segment(&mut self, line_source: &LineSource) {}
    fn push_segment(&mut self, line_source: &LineSource) {
        assert!(line_source.tokens.len() == 3);
        let memory_segment = match MemorySegments::from_token(line_source.tokens[1].as_str()) {
            Ok(memory_segment) => memory_segment,
            Err(msg) => return self.error(line_source.line, msg),
        };
        let i = match line_source.tokens[2].parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                return self.error(
                    line_source.line,
                    format!("Unknown i at {}", line_source.tokens[2]),
                )
            }
        };
        match memory_segment {
            MemorySegments::Temp => {
                // i should only be 0 - 7
                if i > 7 {
                    return self.error(
                        line_source.line,
                        format!("push temp i, i should be between 0-7 not {}", i),
                    );
                }
            }
            MemorySegments::Pointer => {
                // Should only be 0 or 1
                if i > 1 {
                    return self.error(
                        line_source.line,
                        format!("push pointer i, i should be 0 or 1, not {}", i),
                    );
                }
            }
            _ => {} // The other memory segment types, no need to check
        }
        self.asm.append(&mut CodeGen::push_segment(
            &self.file_name,
            memory_segment,
            i,
        ));
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
    fn error_quad_token() {
        let source = "wow very funny lol";
        let mut compiler = Compiler::new(source.to_string(), "somefile".to_string());
        compiler.run();
        assert!(compiler.had_error)
    }
    #[test]
    fn error_neg_push() {
        let source = "push constant -10";
        let mut compiler = Compiler::new(source.to_string(), "somefile".to_string());
        compiler.run();
        assert!(compiler.had_error)
    }
    #[test]
    fn error_temp_large() {
        let source = "push temp 10";
        let mut compiler = Compiler::new(source.to_string(), "somefile".to_string());
        compiler.run();
        assert!(compiler.had_error)
    }
    #[test]
    fn error_pointer_3() {
        let source = "push pointer 3";
        let mut compiler = Compiler::new(source.to_string(), "somefile".to_string());
        compiler.run();
        assert!(compiler.had_error)
    }
}
