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
                _ => unreachable!(),
            }
        }
        Expression::BinaryOperation(op, left, right) => {
            gen_expression(result, left);
            result.push_str("  str x0, [sp, #-0x10]!\n");
            gen_expression(result, right);
            result.push_str("  ldr x1, [sp], #0x10\n");

            match op {
                Operator::Addition => {
                    result.push_str("  add x0, x1, x0\n");
                }
                Operator::Subtraction => {
                    result.push_str("  sub x0, x1, x0\n");
                }
                Operator::Multiplication => {
                    result.push_str("  mul x0, x1, x0\n");
                }
                Operator::Division => {
                    result.push_str("  sdiv x0, x1, x0\n");
                }
                Operator::Equal => {
                    result.push_str("  cmp x1, x0\n");
                    result.push_str("  cset x0, eq\n");
                }
                Operator::NotEqual => {
                    result.push_str("  cmp x1, x0\n");
                    result.push_str("  cset x0, ne\n");
                }
                Operator::LessThan => {
                    result.push_str("  cmp x1, x0\n");
                    result.push_str("  cset x0, lt\n");
                }
                Operator::LessThanOrEqual => {
                    result.push_str("  cmp x1, x0\n");
                    result.push_str("  cset x0, le\n");
                }
                Operator::GreaterThan => {
                    result.push_str("  cmp x1, x0\n");
                    result.push_str("  cset x0, gt\n");
                }
                Operator::GreaterThanOrEqual => {
                    result.push_str("  cmp x1, x0\n");
                    result.push_str("  cset x0, ge\n");
                }
                _ => unreachable!(),
            }
        }
        _ => todo!(),
    }
}
