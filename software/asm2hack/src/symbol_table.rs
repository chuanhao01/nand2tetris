use std::collections::HashMap;

pub struct SimpleSymbolTable {
    pub table: HashMap<String, usize>,
    current_memory: usize,
}
impl SimpleSymbolTable {
    pub fn new() -> Self {
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
    pub fn insert_instruction_label(&mut self, label: String, value: usize) -> Result<(), String> {
        match self.table.get(&label) {
            Some(_) => Err(format!("Instruction label, {}, already exists", label)),
            None => {
                self.table.insert(label, value);
                Ok(())
            }
        }
    }
    pub fn get_or_insert_memory_label(&mut self, label: String) -> usize {
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

impl Default for SimpleSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
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
