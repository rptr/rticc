pub mod lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        lexer::lex_file(filename);
    }
}
