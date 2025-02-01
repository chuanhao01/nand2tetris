pub enum VM_OPS {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}
impl VM_OPS {
    pub fn to_vm_string(&self) -> String {
        let s = match self {
            Self::ADD => "add",
            Self::SUB => "SUB",
            Self::NEG => "NEG",
            Self::EQ => "EQ",
            Self::GT => "GT",
            Self::LT => "LT",
            Self::AND => "AND",
            Self::OR => "OR",
            Self::NOT => "NOT",
        };
        s.to_string()
    }
}
