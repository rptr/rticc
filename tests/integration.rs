use std::fs;
use std::path::Path;
use std::process::Command;

fn run_test(c_file: &Path, return_value: i32) {
    let compile = Command::new("cargo")
        .args(["run", "--bin", "rticc", "--", c_file.to_str().unwrap()])
        .status()
        .expect("failed to run compiler");

    assert!(compile.success(), "compilation failed for {:?}", c_file);

    let binary = c_file.with_extension("");
    let run = Command::new(&binary)
        .status()
        .expect("failed to run compiled binary");

    assert_eq!(
        run.code().unwrap(),
        return_value,
        "wrong exit code for {:?}",
        c_file
    );

    // TODO Make sure to delete even if test fails
    fs::remove_file(binary).unwrap();
}

macro_rules! make_tests {
    ($dir:literal, $($name:ident , $file:literal => $returnvalue:expr),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                run_test(Path::new(concat!("test_data", $dir, "/", $file)), $returnvalue);
            }
        )*
    }
}

make_tests!("",
    return42, "42.c" => 42,
    negative, "neg.c" => 133,
    logical_neg, "logical_negation.c" => 0,
    logical_neg_zero, "logical_negation_zero.c" => 1,
    bitwise_negation, "bitwise_negation.c" => 245
);
