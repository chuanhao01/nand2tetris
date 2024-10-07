/// Some Terminology
/// EOL = End of Line
/// EOF = End of File
pub mod assembler;
pub mod simple;
pub mod symbol_table;

pub use assembler::SimpleAssembler;
pub use symbol_table::SimpleSymbolTable;
