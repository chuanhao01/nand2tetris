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
            Self::SUB => "sub",
            Self::NEG => "neg",
            Self::EQ => "eq",
            Self::GT => "gt",
            Self::LT => "lt",
            Self::AND => "and",
            Self::OR => "or",
            Self::NOT => "not",
        };
        s.to_string()
    }
}
