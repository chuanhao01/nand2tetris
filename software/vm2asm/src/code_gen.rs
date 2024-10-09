pub struct CodeGen;

const SP: &str = "@SP";

impl CodeGen {
    pub fn add() -> Vec<String> {
        let mut asm = vec![String::from("//add")];
        asm.append(&mut Self::sp_minus_1_load_d());
        asm.append(&mut Self::sp_a_m_minus_1());
        asm.push(String::from("M=M+D // *SP-- = *SP-- + D, *SP-- = X + Y"));
        asm
    }
    pub fn sub() -> Vec<String> {
        let mut asm = vec![String::from("//sub")];
        asm.append(&mut Self::sp_minus_1_load_d());
        asm.append(&mut Self::sp_a_m_minus_1());
        asm.push(String::from("M=M-D // *SP-- = *SP-- + D, *SP-- = X - Y"));
        asm
    }
    pub fn neg() -> Vec<String> {
        vec![
            String::from("//neg"),
            SP.to_string(),
            String::from("A=M-1"),
            String::from("M=-M // *SP-- = -*SP--"),
        ]
    }

    fn sp_minus_1_load_d() -> Vec<String> {
        vec![
            SP.to_string(),
            String::from("AM=M-1 // SP = SP--"),
            String::from("D=M // D = *SP"),
        ]
    }
    fn sp_a_m_minus_1() -> Vec<String> {
        vec![SP.to_string(), String::from("A=M-1")]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn load_asm_file_to_vec(file: &str) -> Vec<String> {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push(format!("tests/code_gen/{}", file));
        let source = fs::read_to_string(file_path).unwrap();
        source
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect::<Vec<String>>()
    }

    #[test]
    fn add() {
        assert_eq!(CodeGen::add(), load_asm_file_to_vec("add.asm"));
    }
    #[test]
    fn sub() {
        assert_eq!(CodeGen::sub(), load_asm_file_to_vec("sub.asm"));
    }
    #[test]
    fn neg() {
        assert_eq!(CodeGen::neg(), load_asm_file_to_vec("neg.asm"));
    }
}
