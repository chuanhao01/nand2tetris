pub struct SimpleAssembler;
impl SimpleAssembler {
    pub fn a_instruction(value: usize) -> Result<[char; 16], String> {
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
    pub fn dest(field: &str) -> Result<[char; 3], String> {
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
    pub fn jump(field: &str) -> Result<[char; 3], String> {
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
    pub fn comp(field: &str) -> Result<[char; 7], String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_instruction() {
        assert_eq!(
            SimpleAssembler::a_instruction(0),
            Ok(['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',])
        );
        assert_eq!(
            SimpleAssembler::a_instruction(77),
            Ok(['0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '1', '0', '1',])
        );
        assert_eq!(
            SimpleAssembler::a_instruction(24576),
            Ok(['0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',])
        );
        assert_eq!(
            SimpleAssembler::a_instruction(32767),
            Ok(['0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',])
        );
    }

    #[test]
    fn test_a_instruction_overflow() {
        let overflow = Err(String::from("Overflow A-Instruction value"));
        assert_eq!(SimpleAssembler::a_instruction(32768), overflow.clone());
        assert_eq!(SimpleAssembler::a_instruction(usize::MAX), overflow.clone());
    }
}
