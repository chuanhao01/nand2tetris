use std::fmt::format;

const SP: &str = "@SP";

#[derive(Debug)]
pub enum MemorySegments {
    Local,
    Argument,
    This,
    That,
    Pointer,
    Temp,
    Constant,
    Static,
}
impl MemorySegments {
    pub fn from_token(token: &str) -> Result<Self, String> {
        match token {
            "local" => Ok(Self::Local),
            "argument" => Ok(Self::Argument),
            "this" => Ok(Self::This),
            "that" => Ok(Self::That),
            "pointer" => Ok(Self::Pointer),
            "temp" => Ok(Self::Temp),
            "constant" => Ok(Self::Constant),
            "static" => Ok(Self::Static),
            _ => Err(String::from("Unknown Memory Segment")),
        }
    }
    fn to_token(&self) -> String {
        match self {
            MemorySegments::Local => String::from("local"),
            MemorySegments::Argument => String::from("argument"),
            MemorySegments::This => String::from("this"),
            MemorySegments::That => String::from("that"),
            MemorySegments::Pointer => String::from("pointer"),
            MemorySegments::Temp => String::from("temp"),
            MemorySegments::Constant => String::from("constant"),
            MemorySegments::Static => String::from("static"),
        }
    }
    fn to_asm(&self) -> String {
        match self {
            MemorySegments::Local => String::from("LCL"),
            MemorySegments::Argument => String::from("ARG"),
            MemorySegments::This => String::from("THIS"),
            MemorySegments::That => String::from("THAT"),
            _ => panic!("Tried to_asm an unkown memory_segment, {:?}", self),
        }
    }
}

#[derive(Default)]
pub struct CodeGen {
    binary_counter: usize,
}
impl CodeGen {
    pub fn add() -> Vec<String> {
        let mut asm = vec![String::from("//add")];
        asm.append(&mut Self::sp_minus_1_load_d());
        asm.append(&mut Self::sp_a_m_minus_1());
        asm.push(String::from("M=D+M // *SP-- = *SP-- + D, *SP-- = X + Y"));
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
    pub fn and() -> Vec<String> {
        let mut asm = vec![String::from("//and")];
        asm.append(&mut Self::sp_minus_1_load_d());
        asm.append(&mut Self::sp_a_m_minus_1());
        asm.push(String::from("M=D&M"));
        asm
    }
    pub fn or() -> Vec<String> {
        let mut asm = vec![String::from("//or")];
        asm.append(&mut Self::sp_minus_1_load_d());
        asm.append(&mut Self::sp_a_m_minus_1());
        asm.push(String::from("M=D|M"));
        asm
    }
    pub fn not() -> Vec<String> {
        vec![
            String::from("//not"),
            SP.to_string(),
            String::from("A=M-1"),
            String::from("M=!M // *SP-- = !*SP--"),
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

    pub fn bin_comp(&mut self, file_name: &String, comp: &str) -> Vec<String> {
        let asm_jump_comp = match comp {
            "eq" => "JEQ",
            "gt" => "JGT",
            "lt" => "JLT",
            _ => panic!("Unexpected comp, {}", comp),
        };
        self.binary_counter += 1;
        vec![
            format!("//{}", comp),
            SP.to_string(),
            String::from("AM=M-1 // SP = SP--"),
            String::from("D=M // D = *SP"),
            String::from("A=A-1 // SP--"),
            String::from("D=M-D // D = *SP-- - D, D=x-y"),
            String::from("M=-1 // *SP-- = true"),
            format!("@{}.{}.{}", file_name, comp, self.binary_counter - 1),
            format!("D;{}", asm_jump_comp),
            SP.to_string(),
            String::from("A=M-1"),
            String::from("M=0 // *SP-- = false"),
            format!("({}.{}.{})", file_name, comp, self.binary_counter - 1),
        ]
    }

    pub fn push_segment(
        file_name: &String,
        memory_segment: MemorySegments,
        i: usize,
    ) -> Vec<String> {
        let mut asm = vec![format!("//push {} {}", memory_segment.to_token(), i)];
        match memory_segment {
            MemorySegments::Local
            | MemorySegments::Argument
            | MemorySegments::That
            | MemorySegments::This => {
                asm.append(&mut vec![
                    format!("@{}", memory_segment.to_asm()),
                    String::from("D=M"),
                    format!("@{}", i),
                    String::from("A=D+A"),
                    format!("D=M // D = *({}+{})", memory_segment.to_asm(), i),
                    SP.to_string(),
                    String::from("A=M"),
                    String::from("M=D // *SP = D"),
                    SP.to_string(),
                    String::from("M=M+1 // SP++"),
                ]);
            }
            MemorySegments::Constant => {
                asm.append(&mut vec![
                    format!("@{}", i),
                    format!("D=A // D = {}", i),
                    SP.to_string(),
                    String::from("AM=M+1 // *SP+1, SP++"),
                    String::from("A=A-1"),
                    format!("M=D // *SP = {}", i),
                ]);
            }
            MemorySegments::Static => {
                asm.append(&mut vec![
                    format!("@{}.{}", file_name, i),
                    String::from("D=M"),
                    SP.to_string(),
                    String::from("M=M+1 // SP++"),
                    String::from("A=M-1"),
                    String::from("M=D // *SP = D"),
                ]);
            }
            MemorySegments::Temp => {
                let temp_address = 5 + i;
                asm.append(&mut vec![
                    format!("@{}", temp_address),
                    String::from("D=M"),
                    SP.to_string(),
                    String::from("M=M+1 // SP++"),
                    String::from("A=M-1"),
                    String::from("M=D // *SP = D"),
                ]);
            }
            MemorySegments::Pointer => {
                let pointer = match i {
                    0 => "THIS",
                    1 => "THAT",
                    _ => panic!("Pointer i should be 0 or 1, not {}", i),
                };
                asm.append(&mut vec![
                    format!("@{}", pointer),
                    String::from("D=M"),
                    SP.to_string(),
                    String::from("M=M+1 // SP++"),
                    String::from("A=M-1"),
                    String::from("M=D // *SP = D"),
                ]);
            }
        };
        asm
    }
    pub fn pop_segment(
        file_name: &String,
        memory_segment: MemorySegments,
        i: usize,
    ) -> Vec<String> {
        let mut asm = vec![format!("//pop {} {}", memory_segment.to_token(), i)];
        match memory_segment {
            MemorySegments::Temp => {
                let temp_address = i + 5;
                asm.append(&mut Self::sp_minus_1_load_d());
                asm.append(&mut vec![format!("@{}", temp_address), String::from("M=D")]);
            }
            MemorySegments::Static => {
                asm.append(&mut Self::sp_minus_1_load_d());
                asm.append(&mut vec![
                    format!("@{}.{}", file_name, i),
                    String::from("M=D"),
                ]);
            }
            MemorySegments::Pointer => {
                let pointer = match i {
                    0 => "THIS",
                    1 => "THAT",
                    _ => panic!("Pointer i should be 0 or 1, not {}", i),
                };
                asm.append(&mut Self::sp_minus_1_load_d());
                asm.append(&mut vec![format!("@{}", pointer), String::from("M=D")]);
            }
            MemorySegments::Constant => {
                panic!("Pop Constant should not happen")
            }
            MemorySegments::Local
            | MemorySegments::Argument
            | MemorySegments::That
            | MemorySegments::This => {
                asm.append(&mut Self::sp_minus_1_load_d());
                asm.append(&mut vec![
                    format!("@{}", memory_segment.to_asm()),
                    String::from("D=D+M // *SP + LCL"),
                    format!("@{}", i),
                    String::from("D=D+A // *SP + (LCL+i)"),
                    SP.to_string(),
                    String::from("A=M // *SP"),
                    String::from("A=M // A = *SP"),
                    String::from("A=D-A // A = *SP + (LCL+i) - *SP, A = (LCL+i)"),
                    String::from("M=D-A // *(LCL+i) = *SP + (LCL+i) - (LCL+i)"),
                ]);
            }
        };
        asm
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
    #[test]
    fn push_local_3() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::Local, 3),
            load_asm_file_to_vec("push_local_3.asm")
        );
    }
    #[test]
    fn push_that_4() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::That, 4),
            load_asm_file_to_vec("push_that_4.asm")
        );
    }
    #[test]
    fn push_constant_4() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::Constant, 4),
            load_asm_file_to_vec("push_constant_4.asm")
        );
    }
    #[test]
    fn push_f_static_4() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::Static, 4),
            load_asm_file_to_vec("push_f_static_4.asm")
        );
    }
    #[test]
    fn push_pointer_1() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::Pointer, 1),
            load_asm_file_to_vec("push_pointer_1.asm")
        );
    }
    #[test]
    fn push_temp_3() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::Temp, 3),
            load_asm_file_to_vec("push_temp_3.asm")
        );
    }
    #[test]
    fn eq_f_0() {
        let mut code_gen = CodeGen::default();
        assert_eq!(
            code_gen.bin_comp(&String::from("f"), "eq"),
            load_asm_file_to_vec("eq_f_0.asm")
        );
    }
    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn lt_f_10() {
        let mut code_gen = CodeGen::default();
        code_gen.binary_counter = 10;
        assert_eq!(
            code_gen.bin_comp(&String::from("f"), "lt"),
            load_asm_file_to_vec("lt_f_10.asm")
        );
    }
    #[test]
    fn and() {
        assert_eq!(CodeGen::and(), load_asm_file_to_vec("and.asm"));
    }
    #[test]
    fn or() {
        assert_eq!(CodeGen::or(), load_asm_file_to_vec("or.asm"));
    }
    #[test]
    fn not() {
        assert_eq!(CodeGen::not(), load_asm_file_to_vec("not.asm"));
    }
    #[test]
    fn pop_temp_3() {
        assert_eq!(
            CodeGen::pop_segment(&String::from("f"), MemorySegments::Temp, 3),
            load_asm_file_to_vec("pop_temp_3.asm")
        );
    }
    #[test]
    fn pop_f_static_4() {
        assert_eq!(
            CodeGen::pop_segment(&String::from("f"), MemorySegments::Static, 4),
            load_asm_file_to_vec("pop_f_static_4.asm")
        );
    }
    #[test]
    fn pop_pointer_1() {
        assert_eq!(
            CodeGen::pop_segment(&String::from("f"), MemorySegments::Pointer, 1),
            load_asm_file_to_vec("pop_pointer_1.asm")
        );
    }
    #[test]
    #[should_panic]
    fn pop_constant_3() {
        CodeGen::pop_segment(&String::from("f"), MemorySegments::Constant, 3);
    }
    #[test]
    fn pop_local_3() {
        assert_eq!(
            CodeGen::pop_segment(&String::from("f"), MemorySegments::Local, 3),
            load_asm_file_to_vec("pop_local_3.asm")
        );
    }
}
