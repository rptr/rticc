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
    let t = parser.next();

    if t.is_none() {
        panic!("unexpected end of input while parsing expression");
    }

    let expr = match t.unwrap() {
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
        _ => panic!("unexpected token in expression: {:?}", t),
    };

    expr
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
            _ => panic!("expected Return(IntegerLiteral(0))"),
        }
    }
}
