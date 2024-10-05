use std::collections::HashMap;

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

struct SimpleAssembler;
impl SimpleAssembler {
    fn a_instruction(value: usize) -> Result<[char; 16], String> {
        // Should not overflow, 32767, 2^15 -1
        if value > 32767 {
            return Err(String::from("Overflow A-Instruction value"));
        }
        // Try to cast the vec, should work
        let instruction: [char; 16] = format!("0{value:015b}")
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        Ok(instruction)
    }
    fn dest(field: &str) -> Result<[char; 3], String> {
        match field {
            "null" => Ok(['0'; 3]),
            "M" => Ok(['0', '0', '1']),
            "D" => Ok(['0', '1', '0']),
            "MD" => Ok(['0', '1', '1']),
            "A" => Ok(['1', '0', '0']),
            "AM" => Ok(['1', '0', '1']),
            "AD" => Ok(['1', '1', '0']),
            "AMD" => Ok(['1'; 3]),
            _ => Err(String::from("Invalid dest field")),
        }
    }
    fn jump(field: &str) -> Result<[char; 3], String> {
        match field {
            "null" => Ok(['0'; 3]),
            "JGT" => Ok(['0', '0', '1']),
            "JEQ" => Ok(['0', '1', '0']),
            "JGE" => Ok(['0', '1', '1']),
            "JLT" => Ok(['1', '0', '0']),
            "JNE" => Ok(['1', '0', '1']),
            "JLE" => Ok(['1', '1', '0']),
            "JMP" => Ok(['1'; 3]),
            _ => Err(String::from("Invalid jump field")),
        }
    }
    fn comp(field: &str) -> Result<[char; 7], String> {
        match field {
            "0" => Ok(['0', '1', '0', '1', '0', '1', '0']),
            "1" => Ok(['0', '1', '1', '1', '1', '1', '1']),
            "-1" => Ok(['0', '1', '1', '1', '0', '1', '0']),
            "D" => Ok(['0', '0', '0', '1', '1', '0', '0']),
            "A" => Ok(['0', '1', '1', '0', '0', '0', '0']),
            "M" => Ok(['1', '1', '1', '0', '0', '0', '0']),
            "!D" => Ok(['0', '0', '0', '1', '1', '0', '1']),
            "!A" => Ok(['0', '1', '1', '0', '0', '0', '1']),
            "!M" => Ok(['1', '1', '1', '0', '0', '0', '1']),
            "-D" => Ok(['0', '0', '0', '1', '1', '1', '1']),
            "-A" => Ok(['0', '1', '1', '0', '0', '1', '1']),
            "-M" => Ok(['1', '1', '1', '0', '0', '1', '1']),
            "D+1" => Ok(['0', '0', '1', '1', '1', '1', '1']),
            "A+1" => Ok(['0', '1', '1', '0', '1', '1', '1']),
            "M+1" => Ok(['1', '1', '1', '0', '1', '1', '1']),
            "D-1" => Ok(['0', '0', '0', '1', '1', '1', '0']),
            "A-1" => Ok(['0', '1', '1', '0', '0', '1', '0']),
            "M-1" => Ok(['1', '1', '1', '0', '0', '1', '0']),
            "D+A" => Ok(['0', '0', '0', '0', '0', '1', '0']),
            "D+M" => Ok(['1', '0', '0', '0', '0', '1', '0']),
            "D-A" => Ok(['0', '0', '1', '0', '0', '1', '1']),
            "D-M" => Ok(['1', '0', '1', '0', '0', '1', '1']),
            "A-D" => Ok(['0', '0', '0', '0', '1', '1', '1']),
            "M-D" => Ok(['1', '0', '0', '0', '1', '1', '1']),
            "D&A" => Ok(['0', '0', '0', '0', '0', '0', '0']),
            "D&M" => Ok(['1', '0', '0', '0', '0', '0', '0']),
            "D|A" => Ok(['0', '0', '1', '0', '1', '0', '1']),
            "D|M" => Ok(['1', '0', '1', '0', '1', '0', '1']),
            _ => Err(String::from("Invalid comp field")),
        }
    }
}

struct SimpleSymbolTable {
    table: HashMap<String, usize>,
    current_memory: usize,
}
impl SimpleSymbolTable {
    fn new() -> Self {
        let table = HashMap::from([
            (String::from("SP"), 0),
            (String::from("LCL"), 1),
            (String::from("ARG"), 2),
            (String::from("THIS"), 3),
            (String::from("THAT"), 4),
            (String::from("R0"), 0),
            (String::from("R1"), 1),
            (String::from("R2"), 2),
            (String::from("R3"), 3),
            (String::from("R4"), 4),
            (String::from("R5"), 5),
            (String::from("R6"), 6),
            (String::from("R7"), 7),
            (String::from("R8"), 8),
            (String::from("R9"), 9),
            (String::from("R10"), 10),
            (String::from("R11"), 11),
            (String::from("R12"), 12),
            (String::from("R13"), 13),
            (String::from("R14"), 14),
            (String::from("R15"), 15),
            (String::from("SCREEN"), 16384),
            (String::from("KBD"), 24576),
        ]);
        Self {
            table,
            current_memory: 16,
        }
    }
    fn insert_instruction_label(&mut self, label: String, value: usize) -> Result<(), String> {
        match self.table.get(&label) {
            Some(_) => Err(format!("Instruction label, {}, already exists", label)),
            None => {
                self.table.insert(label, value);
                Ok(())
            }
        }
    }
    fn get_or_insert_memory_label(&mut self, label: String) -> usize {
        match self.table.get(&label) {
            // Could also be used to get instruction labels
            Some(value) => *value,
            None => {
                self.table.entry(label).or_insert(self.current_memory);
                self.current_memory += 1;
                self.current_memory - 1
            }
        }
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

    fn is_valid_label(label: &Vec<char>) -> bool {
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
        for i in 1..label.len() {
            if !(label[i].is_ascii_alphanumeric()
                || label[0] == '_'
                || label[0] == '.'
                || label[0] == '$'
                || label[0] == ':')
            {
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
        println!("Error on line {}: {}", line, msg)
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
            let labels = vec!["_", ".", "$", ":", "a2", ".2"];
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
                "(valid)", "(VALID)", "(.)", "($)", "(_)", "(:)", "(v2l1d)", "(_wow)",
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
                simple.a_instruction(&source);
                assert_eq!(
                    simple.rom[0].iter().collect::<String>(),
                    correct_rom_instruction.to_owned()
                );
            }
        }
    }

    mod simple_symbol_table {
        use super::SimpleSymbolTable;

        #[test]
        fn insert_duplicate_instruction() {
            let mut symbol_table = SimpleSymbolTable::new();
            let label = String::from("Again");
            assert_eq!(
                symbol_table.insert_instruction_label(label.clone(), 1),
                Ok(())
            );
            assert!(symbol_table
                .insert_instruction_label(label.clone(), 2)
                .is_err())
        }
        #[test]
        fn insert_reserved_instruction() {
            let mut symbol_table = SimpleSymbolTable::new();
            let label = String::from("R0");
            assert!(symbol_table
                .insert_instruction_label(label.clone(), 2)
                .is_err())
        }
        #[test]
        fn get_reserved_instruction() {
            let mut symbol_table = SimpleSymbolTable::new();
            let label = String::from("R10");
            assert_eq!(symbol_table.get_or_insert_memory_label(label.clone()), 10)
        }
    }

    mod simple_assembler {
        use super::SimpleAssembler;

        #[test]
        fn test_a_instruction() {
            assert_eq!(
                SimpleAssembler::a_instruction(0),
                Ok([
                    '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
                ])
            );
            assert_eq!(
                SimpleAssembler::a_instruction(77),
                Ok([
                    '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '1', '0', '1',
                ])
            );
            assert_eq!(
                SimpleAssembler::a_instruction(24576),
                Ok([
                    '0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',
                ])
            );
            assert_eq!(
                SimpleAssembler::a_instruction(32767),
                Ok([
                    '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',
                ])
            );
        }

        #[test]
        fn test_a_instruction_overflow() {
            let overflow = Err(String::from("Overflow A-Instruction value"));
            assert_eq!(SimpleAssembler::a_instruction(32768), overflow.clone());
            assert_eq!(SimpleAssembler::a_instruction(usize::MAX), overflow.clone());
        }
    }

    #[test]
    fn custom() {}
}
