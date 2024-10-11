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
            MemorySegments::That => String::from("That"),
            _ => panic!("Tried to_asm an unkown memory_segment, {:?}", self),
        }
    }
}

pub struct CodeGen;
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
            _ => {}
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
    fn push_constant_4() {
        assert_eq!(
            CodeGen::push_segment(&String::from("f"), MemorySegments::Constant, 4),
            load_asm_file_to_vec("push_constant_4.asm")
        );
    }
}
