use crate::lexer::{Constant, Keyword, Token};

pub(crate) struct Program {
    pub(crate) func: FunctionDefinition,
}

pub(crate) struct FunctionDefinition {
    pub(crate) name: String,
    pub(crate) body: Vec<Statement>,
}

pub(crate) enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
}

pub(crate) enum Expression {
    IntegerLiteral(u64),
    Identifier(String),
    UnaryOperation(Operator, Box<Expression>),
    BinaryOperation(Operator, Box<Expression>, Box<Expression>),
}

pub(crate) enum Operator {
    NumericNegation,
    LogicalNegation,
    BitwiseNegation,
    Sizeof,
    PrefixIncrement,
    PrefixDecrement,
    AddressOf,
    Dereference,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }

    fn expect(&mut self, expected: &Token) {
        let token = self.next();

        if token != Some(expected) {
            panic!("expected token: {expected:?}, got: {token:?}");
        }
    }
}

pub(crate) fn parse(tokens: Vec<Token>) -> Program {
    let mut parser = Parser { tokens, index: 0 };

    let func_decl = parse_function_definition(&mut parser);

    Program { func: func_decl }
}

fn parse_function_definition(parser: &mut Parser) -> FunctionDefinition {
    parser.expect(&Token::Keyword(Keyword::Int));

    let name = if let Some(Token::Identifier(name)) = parser.peek() {
        name.clone()
    } else {
        panic!("expected function name identifier");
    };

    // Skip name
    parser.next();

    parser.expect(&Token::OpenParen);

    println!("{:?}", parser.peek());

    // optional, ignore
    if parser.peek() == Some(&Token::Keyword(Keyword::Void)) {
        parser.next();
    }

    parser.expect(&Token::CloseParen);
    parser.expect(&Token::OpenCurly);

    let stmt = parse_statement(parser);

    parser.expect(&Token::CloseCurly);

    FunctionDefinition {
        name,
        body: vec![stmt],
    }
}

fn parse_statement(parser: &mut Parser) -> Statement {
    parser.expect(&Token::Keyword(Keyword::Return));

    let t = parser.peek();

    if t == Some(&Token::Semicolon) {
        // return without expression
        parser.next();
        Statement::Return(None)
    } else {
        let expr = parse_expression(parser);

        parser.expect(&Token::Semicolon);

        return Statement::Return(Some(expr));
    }
}

fn parse_expression(parser: &mut Parser) -> Expression {
    let mut term = parse_term(parser);

    while matches!(parser.peek(), Some(Token::Plus) | Some(Token::Minus)) {
        let is_addition = matches!(parser.peek(), Some(Token::Plus));

        parser.next();

        let next_term = parse_term(parser);

        term = if is_addition {
            Expression::BinaryOperation(Operator::Addition, Box::new(term), Box::new(next_term))
        } else {
            Expression::BinaryOperation(Operator::Subtraction, Box::new(term), Box::new(next_term))
        };
    }

    term
}

fn parse_term(parser: &mut Parser) -> Expression {
    let factor = parse_factor(parser);
    let next = parser.peek();

    match next {
        Some(Token::Asterisk) => {
            parser.next();
            let rhs = parse_term(parser);
            Expression::BinaryOperation(Operator::Multiplication, Box::new(factor), Box::new(rhs))
        }
        Some(Token::Division) => {
            parser.next();
            let rhs = parse_term(parser);
            Expression::BinaryOperation(Operator::Division, Box::new(factor), Box::new(rhs))
        }
        _ => factor,
    }
}

fn parse_factor(parser: &mut Parser) -> Expression {
    let t = parser.next();

    if t.is_none() {
        panic!("unexpected end of input while parsing factor");
    }

    match t.unwrap() {
        Token::OpenParen => {
            let expr = parse_expression(parser);
            parser.expect(&Token::CloseParen);
            expr
        }
        Token::Constant(Constant::Int(n)) => Expression::IntegerLiteral(*n),
        Token::Identifier(name) => Expression::Identifier(name.clone()),
        Token::Minus => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::NumericNegation, Box::new(operand))
        }
        Token::LogicalNegation => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::LogicalNegation, Box::new(operand))
        }
        Token::BitwiseNegation => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::BitwiseNegation, Box::new(operand))
        }
        Token::Keyword(Keyword::Sizeof) => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::Sizeof, Box::new(operand))
        }
        Token::PlusPlus => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::PrefixIncrement, Box::new(operand))
        }
        Token::MinusMinus => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::PrefixDecrement, Box::new(operand))
        }
        Token::BitwiseAnd => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::AddressOf, Box::new(operand))
        }
        Token::Asterisk => {
            let operand = parse_expression(parser);
            Expression::UnaryOperation(Operator::Dereference, Box::new(operand))
        }
        _ => panic!("unexpected token in factor: {:?}", t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    #[test]
    fn test_peek() {
        let parser = make_parser(vec![Token::OpenParen, Token::CloseParen]);

        assert_eq!(parser.peek(), Some(&Token::OpenParen));
    }

    #[test]
    fn test_next() {
        let mut parser = make_parser(vec![Token::OpenParen, Token::CloseParen]);

        assert_eq!(parser.next(), Some(&Token::OpenParen));
        assert_eq!(parser.next(), Some(&Token::CloseParen));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_expect() {
        let mut parser = make_parser(vec![Token::Semicolon]);
        parser.expect(&Token::Semicolon);
    }

    #[test]
    #[should_panic(expected = "expected token")]
    fn test_expect_fail() {
        let mut parser = make_parser(vec![Token::OpenParen]);
        parser.expect(&Token::CloseParen);
    }

    #[test]
    fn test_parse_function_definition_basic() {
        let tokens = vec![
            Token::Keyword(Keyword::Int),
            Token::Identifier("hello".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenCurly,
            Token::Keyword(Keyword::Return),
            Token::Constant(Constant::Int(0)),
            Token::Semicolon,
            Token::CloseCurly,
        ];
        let mut parser = make_parser(tokens);
        let decl = parse_function_definition(&mut parser);
        assert_eq!(decl.name, "hello");
    }

    #[test]
    fn test_parse_statement_return_zero() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Constant(Constant::Int(100)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);
        match stmt {
            Statement::Return(Some(Expression::IntegerLiteral(n))) => assert_eq!(n, 100),
            _ => panic!("expected Return(IntegerLiteral(100))"),
        }
    }

    #[test]
    fn test_parse_statement_return_addition() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Constant(Constant::Int(9)),
            Token::Plus,
            Token::Constant(Constant::Int(11)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::BinaryOperation(Operator::Addition, lhs, rhs))) => {
                match (*lhs, *rhs) {
                    (Expression::IntegerLiteral(9), Expression::IntegerLiteral(11)) => {}
                    _ => panic!("expected Addition(9, 11)"),
                }
            }
            _ => panic!("expected Return(Addition(..))"),
        }
    }

    #[test]
    fn test_parse_statement_return_subtraction() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Constant(Constant::Int(10)),
            Token::Minus,
            Token::Constant(Constant::Int(3)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::BinaryOperation(
                Operator::Subtraction,
                lhs,
                rhs,
            ))) => match (*lhs, *rhs) {
                (Expression::IntegerLiteral(10), Expression::IntegerLiteral(3)) => {}
                _ => panic!("expected Subtraction(10, 3)"),
            },
            _ => panic!("expected Return(Subtraction(..))"),
        }
    }

    #[test]
    fn test_parse_statement_return_multiplication() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Constant(Constant::Int(4)),
            Token::Asterisk,
            Token::Constant(Constant::Int(5)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::BinaryOperation(
                Operator::Multiplication,
                lhs,
                rhs,
            ))) => match (*lhs, *rhs) {
                (Expression::IntegerLiteral(4), Expression::IntegerLiteral(5)) => {}
                _ => panic!("expected Multiplication(4, 5)"),
            },
            _ => panic!("expected Return(Multiplication(..))"),
        }
    }

    #[test]
    fn test_parse_statement_return_division() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Constant(Constant::Int(20)),
            Token::Division,
            Token::Constant(Constant::Int(4)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::BinaryOperation(Operator::Division, lhs, rhs))) => {
                match (*lhs, *rhs) {
                    (Expression::IntegerLiteral(20), Expression::IntegerLiteral(4)) => {}
                    _ => panic!("expected Division(20, 4)"),
                }
            }
            _ => panic!("expected Return(Division(..))"),
        }
    }

    #[test]
    fn test_parse_statement_return_mixed_with_parentheses() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::OpenParen,
            Token::Constant(Constant::Int(2)),
            Token::Plus,
            Token::Constant(Constant::Int(3)),
            Token::CloseParen,
            Token::Asterisk,
            Token::Constant(Constant::Int(4)),
            Token::Plus,
            Token::Constant(Constant::Int(5)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::BinaryOperation(Operator::Addition, lhs, rhs))) => {
                match *rhs {
                    Expression::IntegerLiteral(5) => {}
                    _ => panic!("expected right side to be 5"),
                }

                match *lhs {
                    Expression::BinaryOperation(Operator::Multiplication, mult_lhs, mult_rhs) => {
                        match (*mult_lhs, *mult_rhs) {
                            (
                                Expression::BinaryOperation(Operator::Addition, add_lhs, add_rhs),
                                Expression::IntegerLiteral(4),
                            ) => match (*add_lhs, *add_rhs) {
                                (Expression::IntegerLiteral(2), Expression::IntegerLiteral(3)) => {}
                                _ => panic!("expected (2 + 3) inside parentheses"),
                            },
                            _ => panic!("expected Multiplication((2 + 3), 4)"),
                        }
                    }
                    _ => panic!("expected left side to be multiplication"),
                }
            }
            _ => panic!("expected Return(Addition(Multiplication(..), 5))"),
        }
    }

    #[test]
    fn test_parse_unary_numeric_negation() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Minus,
            Token::Constant(Constant::Int(42)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::NumericNegation));
                match *expr {
                    Expression::IntegerLiteral(42) => {}
                    _ => panic!("expected IntegerLiteral(42) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(NumericNegation, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_logical_negation() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::LogicalNegation,
            Token::Constant(Constant::Int(1)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::LogicalNegation));
                match *expr {
                    Expression::IntegerLiteral(1) => {}
                    _ => panic!("expected IntegerLiteral(1) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(LogicalNegation, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_bitwise_negation() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::BitwiseNegation,
            Token::Constant(Constant::Int(15)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::BitwiseNegation));
                match *expr {
                    Expression::IntegerLiteral(15) => {}
                    _ => panic!("expected IntegerLiteral(15) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(BitwiseNegation, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_sizeof() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Keyword(Keyword::Sizeof),
            Token::Constant(Constant::Int(10)),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::Sizeof));
                match *expr {
                    Expression::IntegerLiteral(10) => {}
                    _ => panic!("expected IntegerLiteral(10) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(Sizeof, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_prefix_increment() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::PlusPlus,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::PrefixIncrement));
                match *expr {
                    Expression::Identifier(ref name) => assert_eq!(name, "x"),
                    _ => panic!("expected Identifier(x) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(PrefixIncrement, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_prefix_decrement() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::MinusMinus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::PrefixDecrement));
                match *expr {
                    Expression::Identifier(ref name) => assert_eq!(name, "y"),
                    _ => panic!("expected Identifier(y) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(PrefixDecrement, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_address_of() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::BitwiseAnd,
            Token::Identifier("p".to_string()),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::AddressOf));
                match *expr {
                    Expression::Identifier(ref name) => assert_eq!(name, "p"),
                    _ => panic!("expected Identifier(p) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(AddressOf, ..))"),
        }
    }

    #[test]
    fn test_parse_unary_dereference() {
        let tokens = vec![
            Token::Keyword(Keyword::Return),
            Token::Asterisk,
            Token::Identifier("p".to_string()),
            Token::Semicolon,
        ];
        let mut parser = make_parser(tokens);
        let stmt = parse_statement(&mut parser);

        match stmt {
            Statement::Return(Some(Expression::UnaryOperation(op, expr))) => {
                assert!(matches!(op, Operator::Dereference));
                match *expr {
                    Expression::Identifier(ref name) => assert_eq!(name, "p"),
                    _ => panic!("expected Identifier(p) as operand"),
                }
            }
            _ => panic!("expected Return(UnaryOperation(Dereference, ..))"),
        }
    }
}
