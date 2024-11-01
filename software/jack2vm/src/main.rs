use std::{env, fs, path::Path};

type ProgResult = Result<(), String>;

fn compile_file(file_path: &str) -> ProgResult {
    let file_path = Path::new(file_path);
    // For now we ignore the folder struct to the file and just check that the file extension is .asm
    if file_path.file_name().is_none() {
        return Err(format!("Expected a proper file path, not {:?}", file_path));
    }
    match file_path.extension() {
        Some(extension) => {
            if extension != "jack" {
                return Err(String::from("File does not end with a .jack extension"));
            }
        }
        None => {
            return Err(format!(
                "Expected file, {}, to have .jack extension",
                file_path.file_name().unwrap().to_str().unwrap(),
            ))
        }
    }
    let source = fs::read_to_string(file_path).expect("Read the file contents");

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
