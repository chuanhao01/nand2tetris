use crate::{SimpleAssembler, SimpleSymbolTable};

#[derive(Clone, Debug)]
struct LineSource {
    source: String,
    line: usize,
}
impl LineSource {
    fn new(source: String, line: usize) -> Self {
        Self { source, line }
    }
}

pub struct Simple {
    rom: Vec<[char; 16]>,
    line_sources: Vec<LineSource>,
    symbol_table: SimpleSymbolTable,
    had_error: bool,
}

impl Simple {
    fn new(source: String) -> Self {
        let source = source
            .split('\n')
            .enumerate()
            .map(|(i, s)| LineSource::new(s.to_string(), i))
            .collect::<Vec<LineSource>>();
        Self {
            rom: Vec::default(),
            line_sources: source,
            symbol_table: SimpleSymbolTable::new(),
            had_error: false,
        }
    }
    pub fn compile(source: String) -> Option<Vec<[char; 16]>> {
        let mut simple = Self::new(source);
        simple.remove_whtiespace();
        simple.first_pass();
        simple.hack();
        #[cfg(feature = "debug")]
        {
            println!("{:?}", simple.symbol_table.table);
        }
        if simple.had_error {
            None
        } else {
            Some(simple.rom)
        }
    }

    fn first_pass(&mut self) {
        let mut new_line_sources: Vec<LineSource> = Vec::new();
        let mut rom_line = 0;
        for line_source in self.line_sources.clone() {
            let source = &line_source.source;
            if source.starts_with('(') && source.ends_with(')') {
                // If we encounter an instruction label, remove it and save the symbol of ROM line
                self.add_instruction_label(&line_source, rom_line);
            } else {
                // Normal instruction, increment rom_line number and save line_source
                rom_line += 1;
                new_line_sources.push(line_source);
            }
        }
        self.line_sources = new_line_sources;
    }

    fn hack(&mut self) {
        for line_source in self.line_sources.clone() {
            if line_source.source.starts_with('@') {
                // A-Instruction
                self.a_instruction(&line_source);
            } else {
                // C-Instruction
                self.c_instruction(&line_source);
            }
        }
    }

    fn a_instruction(&mut self, line_source: &LineSource) {
        let source = match line_source.source.strip_prefix('@') {
            Some(s) => s,
            None => {
                return self.error(line_source.line, String::from("Invalid A-Instruction"));
            }
        };
        if source.is_empty() {
            // Empty @ instruction
            return self.error(line_source.line, String::from("Empty A-Instruction"));
        }
        let label = source.chars().collect::<Vec<char>>();
        let value = if Self::is_valid_label(&label) {
            self.symbol_table
                .get_or_insert_memory_label(label.iter().collect::<String>())
        } else {
            match label.iter().collect::<String>().parse::<usize>() {
                Ok(v) => v,
                Err(_) => {
                    return self.error(
                        line_source.line,
                        String::from("Invalid A-Instruction Decimal Value"),
                    );
                }
            }
        };
        match SimpleAssembler::a_instruction(value) {
            Err(msg) => self.error(line_source.line, msg),
            Ok(rom_instruction) => self.rom.push(rom_instruction),
        }
    }

    fn c_instruction(&mut self, line_source: &LineSource) {
        let mut source = line_source.source.clone();
        let null = String::from("null");

        let mut dest = null.clone();
        let comp: String;
        let mut jump = null.clone();

        if source.contains('=') {
            let s = source.split_once('=').unwrap();
            dest = s.0.to_string();
            source = s.1.to_string();
        }
        if source.contains(';') {
            let s = source.split_once(';').unwrap();
            comp = s.0.to_string();
            jump = s.1.to_string();
        } else {
            comp = source;
        }

        let dest_instruction = match SimpleAssembler::dest(dest.trim()) {
            Ok(v) => v,
            Err(msg) => return self.error(line_source.line, msg),
        };
        let comp_instruction = match SimpleAssembler::comp(comp.trim()) {
            Ok(v) => v,
            Err(msg) => return self.error(line_source.line, msg),
        };
        let jump_instruction = match SimpleAssembler::jump(jump.trim()) {
            Ok(v) => v,
            Err(msg) => return self.error(line_source.line, msg),
        };
        let mut instruction = ['1'; 16];
        instruction[13..16].copy_from_slice(&jump_instruction);
        instruction[10..13].copy_from_slice(&dest_instruction);
        instruction[3..10].copy_from_slice(&comp_instruction);
        self.rom.push(instruction);
    }

    fn add_instruction_label(&mut self, line_source: &LineSource, value: usize) {
        let invalid_instruction_msg = String::from("Not a valid instruction label");
        let mut source = match line_source.source.strip_prefix('(') {
            Some(s) => s,
            None => {
                return self.error(line_source.line, invalid_instruction_msg);
            }
        };
        source = match source.strip_suffix(')') {
            Some(s) => s,
            None => {
                return self.error(line_source.line, invalid_instruction_msg);
            }
        };
        let label = source.chars().collect::<Vec<char>>();
        if Self::is_valid_label(&label) {
            if let Err(msg) = self
                .symbol_table
                .insert_instruction_label(label.iter().collect::<String>(), value)
            {
                self.error(line_source.line, msg);
            };
        } else {
            self.error(line_source.line, invalid_instruction_msg);
        }
    }

    fn is_valid_label(label: &[char]) -> bool {
        if label.is_empty() {
            return false;
        }
        if !(label[0].is_ascii_alphabetic()
            || label[0] == '_'
            || label[0] == '.'
            || label[0] == '$'
            || label[0] == ':')
        {
            return false;
        }
        for c in &label[1..label.len()] {
            if !(c.is_ascii_alphanumeric() || c == &'_' || c == &'.' || c == &'$' || c == &':') {
                return false;
            }
        }
        true
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

    fn remove_whtiespace(&mut self) {
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
        self.line_sources = new_line_sources;
    }
}
impl Default for Simple {
    fn default() -> Self {
        Self {
            rom: Vec::default(),
            line_sources: Vec::default(),
            symbol_table: SimpleSymbolTable::new(),
            had_error: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod simple {
        use super::{LineSource, Simple};

        #[test]
        fn empty_remove_whitespace() {
            let source = String::from("\n \n  //\n//no way hose ");
            let mut simple = Simple::new(source);
            simple.remove_whtiespace();
            assert_eq!(simple.line_sources.len(), 0);
        }
        #[test]
        fn token_before_comment_remove_whitespace() {
            let source = String::from("@10//wow what is thi");
            let mut simple = Simple::new(source);
            simple.remove_whtiespace();
            assert_eq!(simple.line_sources[0].source.len(), 3);

            let source = String::from("(LOOP) //no");
            let mut simple = Simple::new(source);
            simple.remove_whtiespace();
            assert_eq!(simple.line_sources[0].source.len(), 6);
        }

        #[test]
        fn valid_labels() {
            let labels = vec!["_", ".", "$", ":", "a2", ".2", "DRAW_REACT", "wow"];
            let labels = labels
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for label in labels {
                assert!(Simple::is_valid_label(
                    &label.chars().collect::<Vec<char>>()
                ));
            }
        }
        #[test]
        fn invalid_labels() {
            let labels = vec!["2a", "", "a%", "oops@"];
            let labels = labels
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for label in labels {
                assert!(!Simple::is_valid_label(
                    &label.chars().collect::<Vec<char>>()
                ));
            }
        }

        #[test]
        fn valid_add_instruction_label() {
            let labels = [
                "(valid)",
                "(VALID)",
                "(.)",
                "($)",
                "(_)",
                "(:)",
                "(v2l1d)",
                "(_wow)",
                "(DRAW_REACT)",
            ];
            let labels = labels
                .iter()
                .map(|s| LineSource::new(s.to_string(), 1))
                .collect::<Vec<LineSource>>();
            let mut simple = Simple::default();
            for label in labels {
                simple.add_instruction_label(&label, 1);
                assert!(!simple.had_error);
                let actual_label = label
                    .source
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap();
                assert_eq!(
                    simple
                        .symbol_table
                        .get_or_insert_memory_label(actual_label.to_string()),
                    1
                );
            }
        }
        #[test]
        fn invalid_add_instruction_label() {
            let labels = ["a(valid)", "(VALID).", "(1abv)", "abc", "@no", "()"];
            let labels = labels
                .iter()
                .map(|s| LineSource::new(s.to_string(), 1))
                .collect::<Vec<LineSource>>();
            for label in labels {
                let mut simple = Simple::default();
                assert!(!simple.had_error);
                simple.add_instruction_label(&label, 1);
                assert!(simple.had_error);
            }
        }

        #[test]
        fn invalid_a_instruction() {
            let sources = ["no", "@", "@-10", "@3333333333", "@3.3"];
            let sources = sources
                .iter()
                .map(|s| LineSource::new(s.to_string(), 1))
                .collect::<Vec<LineSource>>();
            for source in sources {
                let mut simple = Simple::default();
                assert!(!simple.had_error);
                simple.a_instruction(&source);
                assert!(simple.had_error);
            }
        }
        #[test]
        fn valid_a_instruction() {
            let sources = ["@1", "@32000", "@f1", "@R1", "@KBD", "@SCREEN"];
            let correct_rom_instructions = [
                "0000000000000001",
                "0111110100000000",
                "0000000000010000",
                "0000000000000001",
                "0110000000000000",
                "0100000000000000",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
            let sources = sources
                .iter()
                .map(|s| LineSource::new(s.to_string(), 1))
                .collect::<Vec<LineSource>>();
            for (source, correct_rom_instruction) in
                sources.iter().zip(correct_rom_instructions.iter())
            {
                let mut simple = Simple::default();
                simple.a_instruction(source);
                assert_eq!(
                    simple.rom[0].iter().collect::<String>(),
                    correct_rom_instruction.to_owned()
                );
            }
        }
    }
}
