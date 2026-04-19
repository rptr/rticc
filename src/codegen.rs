use crate::parser::{Expression, Operator, Program, Statement};

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
            gen_expression(result, expr.as_ref().unwrap());

            result.push_str("  ret\n");
        }
        _ => todo!(),
    }
}

fn gen_expression(result: &mut String, expr: &Expression) {
    match expr {
        Expression::IntegerLiteral(n) => {
            result.push_str(&format!("  mov x0, #{}\n", n));
        }
        Expression::UnaryOperation(op, expr) => {
            // TODO Don't clone
            gen_expression(result, expr);

            match op {
                Operator::NumericNegation => {
                    result.push_str("  neg x0, x0\n");
                }
                Operator::LogicalNegation => {
                    result.push_str("  cmp x0, #0\n");
                    result.push_str("  cset x0, eq\n");
                }
                Operator::BitwiseNegation => {
                    result.push_str("  mvn x0, x0\n");
                }
                Operator::Sizeof => todo!(),
                Operator::PrefixIncrement => todo!(),
                Operator::PrefixDecrement => todo!(),
                Operator::AddressOf => todo!(),
                Operator::Dereference => todo!(),
            }
        }
        _ => todo!(),
    }
}
