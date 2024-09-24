use std::collections::HashMap;

#[derive(Clone)]
struct LineSource {
    source: String,
    line: usize,
}
impl LineSource {
    fn new(source: String, line: usize) -> Self {
        Self { source, line }
    }
}

struct SimpleSymbolTable {
    table: HashMap<String, usize>,
    current_memory: usize,
}
impl SimpleSymbolTable {
    fn new() -> Self {
        Self {
            table: HashMap::default(),
            current_memory: 16,
        }
    }
    fn insert_instruction_label(&mut self, label: String, value: usize) {
        self.table.entry(label).or_insert(value);
    }
    fn insert_memory_label(&mut self, label: String) {
        self.table.entry(label).or_insert(self.current_memory);
        self.current_memory += 1;
    }
}

pub struct Simple {
    line_sources: Vec<LineSource>,
    symbol_table: SimpleSymbolTable,
}

impl Simple {
    pub fn compile(source: String) {
        let source = source
            .split('\n')
            .enumerate()
            .map(|(i, s)| LineSource::new(s.to_string(), i))
            .collect::<Vec<LineSource>>();
        let mut simple = Self {
            line_sources: source,
            symbol_table: SimpleSymbolTable::new(),
        };
        simple.remove_whtiespace();
        simple.first_pass();
    }

    fn first_pass(&mut self) {
        let mut new_line_sources: Vec<LineSource> = Vec::new();
        for line_source in self.line_sources.clone() {
            let mut source = line_source.source;
            if source.starts_with('(') && source.ends_with(')') {
                // Taking in instruction label
                let chars = source.chars().collect::<Vec<char>>();
                // let chars = chars[1..chars.len()]
            }
        }
    }

    fn remove_whtiespace(&mut self) -> Vec<LineSource> {
        let mut new_line_sources: Vec<LineSource> = Vec::new();
        for line_source in self.line_sources.clone() {
            let mut source = line_source.source;
            if source.contains("//") {
                source = source
                    .split("//")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[0]
                    .clone();
            }
            source = source.trim().to_string();
            if source.is_empty() {
                continue;
            }
            new_line_sources.push(LineSource::new(source, line_source.line));
        }
        new_line_sources
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_remove_whitespace() {
        let source = vec!["", " ", "  //  ", " //no way hose "];
        let source = source
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Simple::remove_whtiespace(source).len(), 0);
    }
    #[test]
    fn token_before_comment_remove_whitespace() {
        let source = vec!["@10//wow what is thi"];
        let source = source
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Simple::remove_whtiespace(source)[0].source.len(), 3);

        let source = vec!["(LOOP) //no"];
        let source = source
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Simple::remove_whtiespace(source)[0].source.len(), 6);
    }

    #[test]
    fn custom() {
        let mut a = SimpleSymbolTable::new();
        a.insert_instruction_label("Wow".to_string(), 15);
        a.insert_instruction_label("Wow".to_string(), 20);
        assert_eq!(a.table.get("Wow").unwrap().to_owned(), 15usize);
    }
}
