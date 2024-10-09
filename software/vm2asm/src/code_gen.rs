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
mod tests {}
