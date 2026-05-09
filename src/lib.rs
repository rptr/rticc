pub mod codegen;
pub mod lexer;
pub mod parser;

use std::process::Command;

pub fn compile(filename: &str) -> Result<(), String> {
    let tokens = lexer::lex_file(filename);
    let ast = parser::parse(tokens);
    let asm = codegen::generate(ast);

    let asm_name = format!("{}.s", filename.strip_suffix(".c").unwrap_or("temp"));
    std::fs::write(&asm_name, &asm).map_err(|e| e.to_string())?;

    let out_name = filename.strip_suffix(".c").unwrap_or("out");

    let output = Command::new("gcc")
        .args([&asm_name, "-o", out_name])
        .output()
        .map_err(|e| format!("could not execute gcc: {}", e))?;

    std::fs::remove_file(&asm_name).map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "gcc failed:\n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ))
    }
}
