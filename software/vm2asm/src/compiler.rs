#[derive(Debug, Clone)]
struct LineSource {
    source: String,
    line: usize,
}

pub struct Compiler {
    line_sources: Vec<LineSource>,
    had_error: bool,
}

impl Compiler {
    fn new(source: String) -> Self {
        let source = source
            .split('\n')
            .enumerate()
            .map(|(i, s)| LineSource {
                source: s.to_string(),
                line: i + 1, // Source file lines start from 1
            })
            .collect::<Vec<LineSource>>();
        Self {
            // rom: Vec::default(),
            line_sources: source,
            // symbol_table: SimpleSymbolTable::new(),
            had_error: false,
        }
    }

    pub fn compile(source: String) -> Option<Vec<String>> {
        let mut compiler = Self::new(source);
        Some(Vec::default())
    }

    fn remove_whtiespace(&mut self) {
        let mut new_line_sources: Vec<LineSource> = Vec::new();
        for line_source in self.line_sources.clone() {
            let mut source = line_source.source;
            // Strip out comments part
            if source.contains("//") {
                source = source
                    .split("//")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[0]
                    .clone();
            }
            // Trim all remaining whitespace
            source = source.trim().to_string();
            // If its an empty line, we skip it
            if source.is_empty() {
                continue;
            }
            new_line_sources.push(LineSource {
                source,
                line: line_source.line,
            });
        }
        self.line_sources = new_line_sources;
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
    fn empty_remove_whitespace() {
        let source = String::from("\n \n  //\n//no way hose ");
        let mut compiler = Compiler::new(source);
        compiler.remove_whtiespace();
        assert_eq!(compiler.line_sources.len(), 0);
    }
    #[test]
    fn token_before_comment_remove_whitespace() {
        let source = String::from("@10//wow what is thi");
        let mut compiler = Compiler::new(source);
        compiler.remove_whtiespace();
        assert_eq!(compiler.line_sources[0].source, "@10".to_string());

        let source = String::from("(LOOP) //no");
        let mut compiler = Compiler::new(source);
        compiler.remove_whtiespace();
        assert_eq!(compiler.line_sources[0].source, "(LOOP)".to_string());

        let source = String::from("push local 2 //no");
        let mut compiler = Compiler::new(source);
        compiler.remove_whtiespace();
        assert_eq!(compiler.line_sources[0].source, "push local 2".to_string());
    }
}
