use std::{env, fs, path::Path};

use jack2vm::Parser;

type ProgResult = Result<(), String>;

fn compile_jack_to_vm(file_path: &Path) -> ProgResult {
    for entry in fs::read_dir(file_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let entry_path = entry.path();
        if !entry_path.is_file() {
            // Skip folders
            continue;
        }
        if let Some(extension) = entry_path.extension() {
            // Skip non .jack files
            if extension != "jack" {
                continue;
            }
        }
        let source = fs::read_to_string(entry_path.clone()).expect("Read the file contents");
        let parser_code_output = Parser::parse(&source).map_err(|e| {
            format!(
                "Compilation error for file {}\n{}",
                entry_path.to_str().unwrap(),
                e
            )
        })?;
        #[cfg(feature = "xml")]
        {
            let mut ast_file_path = entry_path.to_path_buf();
            ast_file_path.set_extension("xml");
            fs::write(ast_file_path, parser_code_output).map_err(|e| e.to_string())?;
        }
        let mut vm_file_path = entry_path.to_path_buf();
        vm_file_path.set_extension("vm");
        fs::write(vm_file_path, parser_code_output.vm).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn compile_folder(file_path: &str) -> ProgResult {
    let file_path = Path::new(file_path);
    // Iterate through all the files in the directory
    if !file_path.is_dir() {
        return Err(String::from("Not a directory"));
    }
    compile_jack_to_vm(file_path)?;

    Ok(())
}

/// Compiles a single .asm file, outputs the binary as outputs/*.hack
fn main() -> ProgResult {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // run main prog
        if let Err(e) = compile_folder(&args[1]) {
            println!("{}", e);
        }
        Ok(())
    } else {
        Err(String::from("Usage: jack2vm [path]"))
    }
}
