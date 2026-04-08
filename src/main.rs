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

        std::fs::write("temp.s", asm).unwrap();

        let _ = Command::new("gcc")
            .args(["temp.s", "-o", "out"])
            .output()
            .expect("could not execute gcc");

        std::fs::remove_file("temp.s").unwrap();
    }
}
