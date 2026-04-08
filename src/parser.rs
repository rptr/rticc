use crate::lexer::{Constant, Keyword, Token};

pub(crate) struct Program {
    func: FunctionDefinition,
}

pub(crate) struct FunctionDefinition {
    name: String,
    body: Vec<Statement>,
}

pub(crate) enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
}

pub(crate) enum Expression {
    IntegerLiteral(u64),
    Identifier(String),
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
        let t = parser.next();

        let n = if let Some(Token::Constant(Constant::Int(n))) = t {
            *n
        } else {
            panic!("expected integer literal after return");
        };

        parser.expect(&Token::Semicolon);

        return Statement::Return(Some(Expression::IntegerLiteral(n)));
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
            _ => panic!("expected Return(IntegerLiteral(0))"),
        }
    }
}
