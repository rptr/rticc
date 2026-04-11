use crate::parser::{Expression, Program, Statement};

pub(crate) fn generate(program: Program) -> String {
    let mut result = String::new();

    gen_function_definition(&mut result, &program.func);

    result
}

fn gen_function_definition(result: &mut String, func: &crate::parser::FunctionDefinition) {
    result.push_str(&format!(".globl _{}\n", func.name));
    result.push_str(&format!("_{}:\n", func.name));

    for stmt in &func.body {
        gen_statement(result, stmt);
    }
}

fn gen_statement(result: &mut String, stmt: &crate::parser::Statement) {
    match stmt {
        Statement::Return(expr) => {
            if let Some(expr) = expr {
                if let Expression::IntegerLiteral(n) = expr {
                    result.push_str(&format!("  mov x0, #{}\n", n));
                } else {
                    todo!()
                }
            }

            result.push_str("  ret\n");
        }
        _ => todo!(),
    }
}
