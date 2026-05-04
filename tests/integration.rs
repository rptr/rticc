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
        run.code().expect("No exit code"),
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
    bitwise_negation, "bitwise_negation.c" => 245,
    add, "add.c" => 6,
    subtract, "subtract.c" => 13,
    multiply, "multiply.c" => 33,
    divide, "divide.c" => 9,
    equal, "equal.c" => 1,
    not_equal, "not_equal.c" => 1,
    less_than, "less_than.c" => 1,
    less_than_or_equal, "less_than_or_equal.c" => 0,
    greater_than, "greater_than.c" => 0,
    greater_than_or_equal, "greater_than_or_equal.c" => 1,
    logical_and, "logical_and.c" => 0,
    logical_or, "logical_or.c" => 1,
    int, "int.c" => 26,
    int_add, "int_add.c" => 139,
    if_, "if.c" => 100,
    if_else, "if_else.c" => 200,
    scope, "scope.c" => 55,
    inner_scope_affects_outer, "scope_2.c" => 100,
    while_, "while.c" => 7,
    do_, "do.c" => 7,
    for_, "for.c" => 7
);
