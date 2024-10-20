use std::{env, fs, path::Path};

type ProgResult = Result<(), String>;

fn compile_folder(file_path: &str) -> ProgResult {
    let file_path = Path::new(file_path);
    // Iterate through all the files in the directory
    // if file_path.

    // For now we ignore the folder struct to the file and just check that the file extension is .asm
    if file_path.file_name().is_none() {
        return Err(format!("Expected a proper file path, not {:?}", file_path));
    }
    let source = fs::read_to_string(file_path).expect("Read the file contents");
    // match Compiler::compile(
    //     source,
    //     file_path
    //         .file_stem()
    //         .expect("Should have a file stem")
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    // ) {
    //     Some(assembly) => {
    //         // Write file to ouputs
    //         let output_dir = file_path
    //             .parent()
    //             .unwrap()
    //             .parent()
    //             .unwrap()
    //             .join("outputs");
    //         fs::create_dir_all(output_dir.clone()).map_err(|e| e.to_string())?;
    //         let mut output_file =
    //             output_dir.join(file_path.file_stem().expect("Should have a file stem"));
    //         output_file.set_extension("asm");
    //         let mut asm = CodeGen::bootstrap();
    //         asm.append(&mut assembly.clone());
    //         fs::write(output_file, asm.join("\n")).map_err(|e| e.to_string())?
    //     }
    //     None => return Err(String::from("Failed to compile")),
    // };

    Ok(())
}

/// Compiles all .vm files in a given directory
fn main() -> ProgResult {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // run main prog
        compile_folder(&args[1])
    } else {
        Err(String::from("Usage: rust-hackasm2bin [path]"))
    }
}
