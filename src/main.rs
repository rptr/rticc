pub mod codegen;
pub mod lexer;
pub mod parser;

use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        let tokens = lexer::lex_file(filename);
        let ast = parser::parse(tokens);
        let asm = codegen::generate(ast);

        let asm_name = format!("{}.s", filename.strip_suffix(".c").unwrap_or("temp"));
        std::fs::write(&asm_name, asm).unwrap();

        let out_name = filename.strip_suffix(".c").unwrap_or("out");

        match Command::new("gcc")
            .args([&asm_name, "-o", out_name])
            .output()
        {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!("gcc failed:");
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    eprintln!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => eprintln!("could not execute gcc: {}", e),
        }

        // std::fs::remove_file(asm_name).unwrap();
    }
}
