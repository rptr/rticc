fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(filename) = args.get(1) {
        if let Err(e) = rticc::compile(filename) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
