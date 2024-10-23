use std::{env, fs, path::Path};

use asm2hack::simple::Simple;
use vm2asm::{CodeGen, Compiler};

type ProgResult = Result<(), String>;

fn compile_to_asm(file_path: &Path) -> ProgResult {
    for entry in fs::read_dir(file_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let entry_path = entry.path();
        if !entry_path.is_file() {
            // Skip folders
            continue;
        }
        if let Some(extension) = entry_path.extension() {
            if extension != "vm" {
                continue;
            }
        }
        let source = fs::read_to_string(entry_path.clone()).expect("Read the file contents");
        let file_name = entry_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        match Compiler::compile(source, file_name) {
            Some(asm) => {
                let mut asm_file_path = entry_path.clone();
                asm_file_path.set_extension("asm");
                fs::write(asm_file_path, asm.join("\n")).map_err(|e| e.to_string())?
            }
            None => {
                return Err(format!(
                    "Failed to compile {}",
                    entry_path.to_str().unwrap()
                ))
            }
        }
    }
    Ok(())
}

fn compile_to_hack(file_path: &Path) -> ProgResult {
    let program_name = file_path.file_name().unwrap().to_str().unwrap();
    let hack_program_path = file_path.join(format!("{}.hack", program_name));
    let mut sources: Vec<String> = Vec::default();

    for entry in fs::read_dir(file_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let entry_path = entry.path();
        if !entry_path.is_file() {
            // Skip folders
            continue;
        }
        if let Some(extension) = entry_path.extension() {
            if extension != "asm" {
                continue;
            }
        }
        if let Some(entry_file_stem) = entry.path().file_stem() {
            if let Some(file_name) = file_path.file_name() {
                if file_name == entry_file_stem {
                    // Debug asm file, skip
                    continue;
                }
            }
        }
        let source = fs::read_to_string(entry_path.clone()).expect("Read the file contents");
        sources.push(source);
    }

    let mut bootstraped_sources = Vec::default();
    bootstraped_sources.append(&mut Vec::from([
        String::from("@256"),
        String::from("D=A"),
        String::from("@SP"),
        String::from("M=D"),
    ]));
    let mut code_gen = CodeGen::default();
    bootstraped_sources.append(&mut code_gen.call(
        &String::from("bootstrap"),
        &String::from("Sys.init"),
        0,
    ));
    bootstraped_sources.append(&mut sources);

    // Adding bootstrap code
    #[cfg(feature = "debug")]
    {
        let asm_program_path = file_path.join(format!("{}.asm", program_name));
        fs::write(asm_program_path, bootstraped_sources.join("\n")).map_err(|e| e.to_string())?;
    }

    match Simple::compile(bootstraped_sources.join("\n")) {
        Some(rom) => {
            fs::write(
                hack_program_path,
                rom.iter()
                    .map(|line| line.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .map_err(|e| e.to_string())?;
            Ok(())
        }
        None => Err(String::from("Failed to compile to hack")),
    }
}

fn compile_folder(file_path: &str) -> ProgResult {
    let file_path = Path::new(file_path);
    // Iterate through all the files in the directory
    if !file_path.is_dir() {
        return Err(String::from("Not a directory"));
    }
    compile_to_asm(file_path)?;
    compile_to_hack(file_path)?;

    Ok(())
}

/// Compiles all .vm files in a given directory
fn main() -> ProgResult {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // run main prog
        compile_folder(&args[1])
    } else {
        Err(String::from("Usage: rust-vm2hack [path]"))
    }
}
