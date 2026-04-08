pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        let tokens = lexer::lex_file(filename);
        let _ = parser::parse(tokens);
    }
}
