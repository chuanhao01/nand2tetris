/// Some Terminology
/// EOL = End of Line
/// EOF = End of File
pub mod assembler;
pub mod compiler;
pub mod scanner;
pub mod tokens;

pub use tokens::{Token, TokenType};
