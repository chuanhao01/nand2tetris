use crate::{Token, TokenType};

pub struct Assembler;

impl Assembler {
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
    fn parse_number(source: &Vec<char>, start: usize, length: usize) -> usize {
        usize::from_str_radix(
            &source[start..start + length].iter().collect::<String>(),
            10,
        )
        .unwrap()
    }
    pub fn comp(source: &Vec<char>, tokens: &Vec<Token>) -> Result<[char; 7], String> {
        if tokens.len() > 3 || tokens.is_empty() {
            return Err(String::from("Expected correct comp"));
        }
        let error_token = Err(String::from("Error Token"));
        let unexpected_token = Err(String::from("Unexpected Token"));

        if tokens.len() == 1 {
            if let Token::NormalToken {
                _type,
                start,
                length,
                line: _,
            } = &tokens[0]
            {
                match _type {
                    TokenType::Number => {
                        let number =
                            Self::parse_number(source, start.to_owned(), length.to_owned());
                        match number {
                            0 => Ok(['0', '1', '0', '1', '0', '1', '0']),
                            1 => Ok(['0', '1', '1', '1', '1', '1', '1']),
                            _ => Err(String::from("Expected 0 or 1")),
                        }
                    }
                    TokenType::D => Ok(['0', '0', '0', '1', '1', '0', '0']),
                    TokenType::A => Ok(['0', '1', '1', '0', '0', '0', '0']),
                    TokenType::M => Ok(['1', '1', '1', '0', '0', '0', '0']),
                    _ => unexpected_token,
                }
            } else {
                return error_token;
            }
        } else if tokens.len() == 2 {
            if let Token::NormalToken {
                _type: TokenType::Minus,
                start: _,
                length: _,
                line: _,
            } = &tokens[0]
            {
                if let Token::NormalToken {
                    _type,
                    start,
                    length,
                    line: _,
                } = &tokens[1]
                {
                    match _type {
                        TokenType::Number => {
                            let number =
                                Self::parse_number(source, start.to_owned(), length.to_owned());
                            match number {
                                1 => Ok(['0', '1', '1', '1', '0', '1', '0']),
                                _ => Err(String::from("Expected 0 or 1")),
                            }
                        }
                        TokenType::D => Ok(['0', '0', '0', '1', '1', '1', '1']),
                        TokenType::A => Ok(['0', '1', '1', '0', '0', '1', '1']),
                        TokenType::M => Ok(['1', '1', '1', '0', '0', '1', '1']),
                        _ => unexpected_token,
                    }
                } else {
                    return error_token;
                }
            } else if let Token::NormalToken {
                _type: TokenType::Bang,
                start: _,
                length: _,
                line: _,
            } = &tokens[0]
            {
                if let Token::NormalToken {
                    _type,
                    start: _,
                    length: _,
                    line: _,
                } = &tokens[1]
                {
                    match _type {
                        TokenType::D => Ok(['0', '0', '0', '1', '1', '0', '1']),
                        TokenType::A => Ok(['0', '1', '1', '0', '0', '0', '1']),
                        TokenType::M => Ok(['1', '1', '1', '0', '0', '0', '1']),
                        _ => unexpected_token,
                    }
                } else {
                    return error_token;
                }
            } else {
                return error_token;
            }
        } else if tokens.len() == 3 {
            match tokens[0..2] {
                [Token::NormalToken {
                    _type: TokenType::D,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Plus,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start,
                        length,
                        line: _,
                    } => {
                        let number = Self::parse_number(source, start, length);
                        if number == 1 {
                            Ok(['0', '0', '1', '1', '1', '1', '1'])
                        } else {
                            unexpected_token
                        }
                    }
                    Token::NormalToken {
                        _type: TokenType::A,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['0', '0', '0', '0', '0', '1', '0']),
                    Token::NormalToken {
                        _type: TokenType::M,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['1', '0', '0', '0', '0', '1', '0']),
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::D,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Minus,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start,
                        length,
                        line: _,
                    } => {
                        let number = Self::parse_number(source, start, length);
                        if number == 1 {
                            Ok(['0', '0', '0', '1', '1', '1', '0'])
                        } else {
                            unexpected_token
                        }
                    }
                    Token::NormalToken {
                        _type: TokenType::A,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['0', '0', '1', '0', '0', '1', '1']),
                    Token::NormalToken {
                        _type: TokenType::M,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['1', '0', '1', '0', '0', '1', '1']),
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::D,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Or,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::A,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['0', '0', '1', '0', '1', '0', '1']),
                    Token::NormalToken {
                        _type: TokenType::M,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['1', '0', '1', '0', '1', '0', '1']),
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::D,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::And,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::A,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['0', '0', '0', '0', '0', '0', '0']),
                    Token::NormalToken {
                        _type: TokenType::M,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['1', '0', '0', '0', '0', '0', '0']),
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::A,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Plus,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start,
                        length,
                        line: _,
                    } => {
                        let number = Self::parse_number(source, start, length);
                        if number == 1 {
                            Ok(['0', '1', '1', '0', '1', '1', '1'])
                        } else {
                            unexpected_token
                        }
                    }
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::A,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Minus,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start,
                        length,
                        line: _,
                    } => {
                        let number = Self::parse_number(source, start, length);
                        if number == 1 {
                            Ok(['0', '1', '1', '0', '0', '1', '0'])
                        } else {
                            unexpected_token
                        }
                    }
                    Token::NormalToken {
                        _type: TokenType::D,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['0', '0', '0', '0', '1', '1', '1']),
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::M,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Plus,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start,
                        length,
                        line: _,
                    } => {
                        let number = Self::parse_number(source, start, length);
                        if number == 1 {
                            Ok(['1', '1', '1', '0', '1', '1', '1'])
                        } else {
                            unexpected_token
                        }
                    }
                    _ => unexpected_token,
                },
                [Token::NormalToken {
                    _type: TokenType::M,
                    start: _,
                    length: _,
                    line: _,
                }, Token::NormalToken {
                    _type: TokenType::Minus,
                    start: _,
                    length: _,
                    line: _,
                }] => match tokens[2] {
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start,
                        length,
                        line: _,
                    } => {
                        let number = Self::parse_number(source, start, length);
                        if number == 1 {
                            Ok(['1', '1', '1', '0', '0', '1', '0'])
                        } else {
                            unexpected_token
                        }
                    }
                    Token::NormalToken {
                        _type: TokenType::D,
                        start: _,
                        length: _,
                        line: _,
                    } => Ok(['0', '0', '0', '0', '1', '1', '1']),
                    _ => unexpected_token,
                },
                _ => unexpected_token,
            }
        } else {
            Ok(['0', '0', '0', '0', '0', '0', '0'])
        }
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn test_a_instruction() {
        assert_eq!(
            Assembler::a_instruction(0),
            Ok(['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',])
        );
        assert_eq!(
            Assembler::a_instruction(77),
            Ok(['0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '1', '0', '1',])
        );
        assert_eq!(
            Assembler::a_instruction(24576),
            Ok(['0', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0',])
        );
        assert_eq!(
            Assembler::a_instruction(32767),
            Ok(['0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1',])
        );
    }

    #[test]
    fn test_comp() {
        let source = String::from("01");
        let source = source.chars().collect::<Vec<char>>();
        assert_eq!(
            Assembler::comp(
                &source,
                &vec![Token::NormalToken {
                    _type: TokenType::M,
                    start: 0,
                    length: 0,
                    line: 0
                }]
            ),
            Ok(['1', '1', '1', '0', '0', '0', '0'])
        );
        assert_eq!(
            Assembler::comp(
                &source,
                &vec![Token::NormalToken {
                    _type: TokenType::Number,
                    start: 1,
                    length: 1,
                    line: 0
                }]
            ),
            Ok(['0', '1', '1', '1', '1', '1', '1'])
        );
        assert_eq!(
            Assembler::comp(
                &source,
                &vec![
                    Token::NormalToken {
                        _type: TokenType::Minus,
                        start: 0,
                        length: 0,
                        line: 0
                    },
                    Token::NormalToken {
                        _type: TokenType::Number,
                        start: 1,
                        length: 1,
                        line: 0
                    }
                ]
            ),
            Ok(['0', '1', '1', '1', '0', '1', '0'])
        );
    }

    #[test]
    fn test_a_instruction_overflow() {
        let overflow = Err(String::from("Overflow A-Instruction value"));
        assert_eq!(Assembler::a_instruction(32768), overflow.clone());
        assert_eq!(Assembler::a_instruction(usize::MAX), overflow.clone());
    }
}
