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

        let _ = Command::new("gcc")
            .args([&asm_name, "-o", out_name])
            .output()
            .expect("could not execute gcc");

        std::fs::remove_file(asm_name).unwrap();
    }
}
