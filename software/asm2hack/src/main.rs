use std::{env, fs, path::Path};

use asm2hack::simple::Simple;

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
    match Simple::compile(source) {
        Some(rom) => {
            // Write file to ouputs
            let output_dir = file_path
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("outputs");
            fs::create_dir_all(output_dir.clone()).map_err(|e| e.to_string())?;
            let mut output_file =
                output_dir.join(file_path.file_stem().expect("Should have a file stem"));
            output_file.set_extension("hack");
            fs::write(
                output_file,
                rom.iter()
                    .map(|line| line.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .map_err(|e| e.to_string())?
        }
        None => return Err(String::from("Failed to compile")),
    };

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
