use std::{env, fs, path::Path};

use rust_asm2hack::{scanner::Scanner, simple::Simple, Token, TokenType};

type ProgResult = Result<(), String>;

fn compile_file(file_path: &str) -> ProgResult {
    let file_path = Path::new(file_path);
    // For now we ignore the folder struct to the file and just check that the file extension is .asm
    if file_path.file_name().is_none() {
        return Err(format!("Expected a proper file path, not {:?}", file_path));
    }
    match file_path.extension() {
        Some(extension) => {
            if extension != "asm" {
                return Err(String::from("File does not end with a .asm extension"));
            }
        }
        None => {
            return Err(format!(
                "Expected file, {}, to have .asm extension",
                file_path.file_name().unwrap().to_str().unwrap(),
            ))
        }
    }
    let source = fs::read_to_string(file_path).expect("Read have read the file contents");
    let output = Simple::compile(source).unwrap();
    println!(
        "{:?}",
        output
            .iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    fs::write(
        format!(
            "outputs/{}.hack",
            file_path.file_name().unwrap().to_str().unwrap()
        ),
        output
            .iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"),
    );

    // let s = source.chars().collect::<Vec<char>>();
    // let mut scanner = Scanner::new();
    // let mut tokens: Vec<Token> = Vec::new();
    // loop {
    //     let token = scanner.scan_token(&s);
    //     let t = if let Token::NormalToken {
    //         _type,
    //         start: _,
    //         length: _,
    //         line: _,
    //     } = &token
    //     {
    //         matches!(_type, TokenType::EOF)
    //     } else {
    //         false
    //     };
    //     tokens.push(token);
    //     if t {
    //         break;
    //     }
    // }
    // println!("{:?}", tokens);

    Ok(())
}

/// Compiles a single .asm file, outputs the binary as outputs/*.hack
fn main() -> ProgResult {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // run main prog
        compile_file(&args[1])
    } else {
        Err(String::from("Usage: rust-hackasm2bin [path]"))
    }
}
