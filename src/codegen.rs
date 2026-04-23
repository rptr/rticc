use crate::parser::{Expression, Operator, Program, Statement};

struct Codegen {
    pub label_counter: u64,
    pub result: String,
}

pub(crate) fn generate(program: Program) -> String {
    let mut codegen = Codegen {
        label_counter: 0,
        result: String::new(),
    };

    gen_function_definition(&mut codegen, &program.func);

    codegen.result
}

fn gen_function_definition(codegen: &mut Codegen, func: &crate::parser::FunctionDefinition) {
    codegen.result.push_str(&format!(".globl _{}\n", func.name));
    codegen.result.push_str(&format!("_{}:\n", func.name));

    for stmt in &func.body {
        gen_statement(codegen, stmt);
    }
}

fn gen_statement(codegen: &mut Codegen, stmt: &crate::parser::Statement) {
    match stmt {
        Statement::Return(expr) => {
            gen_expression(codegen, expr.as_ref().unwrap());

            codegen.result.push_str("  ret\n");
        }
        _ => todo!(),
    }
}

fn gen_expression(codegen: &mut Codegen, expr: &Expression) {
    match expr {
        Expression::IntegerLiteral(n) => {
            codegen.result.push_str(&format!("  mov x0, #{}\n", n));
        }
        Expression::UnaryOperation(op, expr) => {
            gen_expression(codegen, expr);

            match op {
                Operator::NumericNegation => {
                    codegen.result.push_str("  neg x0, x0\n");
                }
                Operator::LogicalNegation => {
                    codegen.result.push_str("  cmp x0, #0\n");
                    codegen.result.push_str("  cset x0, eq\n");
                }
                Operator::BitwiseNegation => {
                    codegen.result.push_str("  mvn x0, x0\n");
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
            if matches!(op, Operator::LogicalAnd | Operator::LogicalOr) {
                gen_logical_expression(codegen, op, left, right);
                return;
            }

            gen_expression(codegen, left);
            codegen.result.push_str("  str x0, [sp, #-0x10]!\n");
            gen_expression(codegen, right);
            codegen.result.push_str("  ldr x1, [sp], #0x10\n");

            match op {
                Operator::Addition => {
                    codegen.result.push_str("  add x0, x1, x0\n");
                }
                Operator::Subtraction => {
                    codegen.result.push_str("  sub x0, x1, x0\n");
                }
                Operator::Multiplication => {
                    codegen.result.push_str("  mul x0, x1, x0\n");
                }
                Operator::Division => {
                    codegen.result.push_str("  sdiv x0, x1, x0\n");
                }
                Operator::Equal => {
                    codegen.result.push_str("  cmp x1, x0\n");
                    codegen.result.push_str("  cset x0, eq\n");
                }
                Operator::NotEqual => {
                    codegen.result.push_str("  cmp x1, x0\n");
                    codegen.result.push_str("  cset x0, ne\n");
                }
                Operator::LessThan => {
                    codegen.result.push_str("  cmp x1, x0\n");
                    codegen.result.push_str("  cset x0, lt\n");
                }
                Operator::LessThanOrEqual => {
                    codegen.result.push_str("  cmp x1, x0\n");
                    codegen.result.push_str("  cset x0, le\n");
                }
                Operator::GreaterThan => {
                    codegen.result.push_str("  cmp x1, x0\n");
                    codegen.result.push_str("  cset x0, gt\n");
                }
                Operator::GreaterThanOrEqual => {
                    codegen.result.push_str("  cmp x1, x0\n");
                    codegen.result.push_str("  cset x0, ge\n");
                }
                _ => unreachable!(),
            }
        }
        _ => todo!(),
    }
}

fn gen_logical_expression(
    codegen: &mut Codegen,
    op: &Operator,
    left: &Expression,
    right: &Expression,
) {
    let label_1 = random_label(codegen);
    let label_2 = random_label(codegen);

    match op {
        Operator::LogicalOr => {
            gen_expression(codegen, left);
            codegen.result.push_str("  cmp x0, #0\n");
            codegen.result.push_str(&format!("  beq {label_1}\n"));
            codegen.result.push_str("  mov x0, #1\n");
            codegen.result.push_str(&format!("  b {label_2}\n"));
            codegen.result.push_str(&format!("{label_1}:\n"));

            gen_expression(codegen, right);
            codegen.result.push_str("  cmp x0, #0\n");
            codegen.result.push_str("  mov x0, #0\n");
            codegen.result.push_str("  cset x0, ne\n");
            codegen.result.push_str(&format!("{label_2}:\n"));
        }
        Operator::LogicalAnd => {
            gen_expression(codegen, left);
            codegen.result.push_str("  cmp x0, #0\n");
            codegen.result.push_str(&format!("  bne {label_1}\n"));
            codegen.result.push_str(&format!("  b {label_2}\n"));
            codegen.result.push_str(&format!("{label_1}:\n"));

            gen_expression(codegen, right);
            codegen.result.push_str("  cmp x0, #0\n");
            codegen.result.push_str("  mov x0, #0\n");
            codegen.result.push_str("  cset x0, ne\n");
            codegen.result.push_str(&format!("{label_2}:\n"));
        }
        _ => unreachable!(),
    }
}

fn random_label(codegen: &mut Codegen) -> String {
    let label = format!("L{}", codegen.label_counter);
    codegen.label_counter += 1;
    label
}
