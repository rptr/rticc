use std::{fs, iter::Peekable};

#[derive(Debug, PartialEq)]
pub(crate) enum Constant {
    Int(u64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Identifier(String),
    Constant(Constant),
    Keyword(Keyword),
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    OpenBracket,
    CloseBracket,
    Period,
    Arrow,
    PlusPlus,
    MinusMinus,
    BitwiseAnd,
    Asterisk,
    Plus,
    Minus,
    BitwiseNegation,
    LogicalNegation,
    Division,
    Modulo,
    BitwiseLeftShift,
    BitwiseRightShift,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    QuestionMark,
    Colon,
    Assignment,
    MultiplyAssignment,
    DivideAssignment,
    ModuloAssignment,
    AddAssignment,
    SubtractAssignment,
    BitwiseLeftShiftAssignment,
    BitwiseRightShiftAssignment,
    BitwiseAndAssignment,
    BitwiseXorAssignment,
    BitwiseOrAssignment,
    Comma,
    Hash,
    HashHash,
}

pub(crate) fn lex_file(filename: &str) -> Vec<Token> {
    let entire_file = fs::read_to_string(filename).unwrap();

    lex(entire_file)
}

fn lex(s: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            c if c.is_numeric() => number(ch, &mut chars, &mut tokens),
            c if c.is_alphabetic() => identifier(ch, &mut chars, &mut tokens),
            '"' => string_literal(&mut chars, &mut tokens),
            '{' => tokens.push(Token::OpenCurly),
            '}' => tokens.push(Token::CloseCurly),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            '.' => tokens.push(Token::Period),
            ';' => tokens.push(Token::Semicolon),
            '*' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '=' {
                        chars.next();
                        tokens.push(Token::MultiplyAssignment);
                    } else {
                        tokens.push(Token::Asterisk);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '*'");
                }
            }
            '~' => tokens.push(Token::BitwiseNegation),
            '/' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '=' {
                        chars.next();
                        tokens.push(Token::DivideAssignment);
                    } else if *next == '/' || *next == '*' {
                        comment(&mut chars);
                    } else {
                        tokens.push(Token::Division);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '/'");
                }
            }
            '%' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '=' {
                        chars.next();
                        tokens.push(Token::ModuloAssignment);
                    } else {
                        tokens.push(Token::Modulo);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '%'");
                }
            }
            ',' => tokens.push(Token::Comma),
            '^' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '=' {
                        chars.next();
                        tokens.push(Token::BitwiseXorAssignment);
                    } else {
                        tokens.push(Token::BitwiseXor);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '^'");
                }
            }
            '?' => tokens.push(Token::QuestionMark),
            ':' => tokens.push(Token::Colon),
            '&' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '&' {
                        chars.next();
                        tokens.push(Token::LogicalAnd);
                    } else if *next == '=' {
                        chars.next();
                        tokens.push(Token::BitwiseAndAssignment);
                    } else {
                        tokens.push(Token::BitwiseAnd);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '&'");
                }
            }
            '!' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '=' {
                        chars.next();
                        tokens.push(Token::NotEqual);
                    } else {
                        tokens.push(Token::LogicalNegation);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '!'");
                }
            }
            '|' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '|' {
                        chars.next();
                        tokens.push(Token::LogicalOr);
                    } else if *next == '=' {
                        chars.next();
                        tokens.push(Token::BitwiseOrAssignment);
                    } else {
                        tokens.push(Token::BitwiseOr);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '|'");
                }
            }
            '<' => {
                less_than(&mut chars, &mut tokens);
            }
            '>' => {
                greater_than(&mut chars, &mut tokens);
            }
            '=' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '=' {
                        chars.next();
                        tokens.push(Token::Equal);
                    } else {
                        tokens.push(Token::Assignment);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '='");
                }
            }
            '-' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '>' {
                        chars.next();
                        tokens.push(Token::Arrow);
                    } else if *next == '-' {
                        chars.next();
                        tokens.push(Token::MinusMinus);
                    } else if *next == '=' {
                        chars.next();
                        tokens.push(Token::SubtractAssignment);
                    } else {
                        tokens.push(Token::Minus);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '-'");
                }
            }
            '+' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '+' {
                        chars.next();
                        tokens.push(Token::PlusPlus);
                    } else if *next == '=' {
                        chars.next();
                        tokens.push(Token::AddAssignment);
                    } else {
                        tokens.push(Token::Plus);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '+'");
                }
            }
            '#' => {
                let peek = chars.peek();

                if let Some(next) = peek {
                    if *next == '#' {
                        chars.next();
                        tokens.push(Token::HashHash);
                    } else {
                        tokens.push(Token::Hash);
                    }
                } else {
                    panic!("lexer: unexpected end of input after '#'");
                }
            }
            c if c.is_whitespace() => continue,
            _ => panic!("lexing: invalid character: {ch}"),
        }
    }

    tokens
}

fn less_than(chars: &mut Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
    let peek = chars.peek();

    if let Some(next) = peek {
        if *next == '<' {
            chars.next();

            if chars.peek() == Some(&'=') {
                chars.next();

                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::BitwiseLeftShiftAssignment);
                } else {
                    panic!("lexer: unexpected character after '<<=': expected '='");
                }
            } else {
                tokens.push(Token::BitwiseLeftShift);
            }
        } else if *next == '=' {
            chars.next();
            tokens.push(Token::LessThanOrEqual);
        } else {
            tokens.push(Token::LessThan);
        }
    } else {
        panic!("lexer: unexpected end of input after '<'");
    }
}

fn greater_than(chars: &mut Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
    let peek = chars.peek();
    if let Some(next) = peek {
        if *next == '>' {
            chars.next();

            if chars.peek() == Some(&'=') {
                chars.next();

                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::BitwiseRightShiftAssignment);
                } else {
                    panic!("lexer: unexpected character after '>>=': expected '='");
                }
            } else {
                tokens.push(Token::BitwiseRightShift);
            }
        } else if *next == '=' {
            chars.next();
            tokens.push(Token::GreaterThanOrEqual);
        } else {
            tokens.push(Token::GreaterThan);
        }
    } else {
        panic!("lexer: unexpected end of input after '>'");
    }
}

fn number(ch: char, chars: &mut Peekable<impl Iterator<Item = char>>, tokens: &mut Vec<Token>) {
    let mut number_str = ch.to_string();

    while let Some(ch) = chars.peek() {
        let ch = *ch;

        if ch.is_numeric() {
            number_str += ch.to_string().as_str();
            chars.next();
        } else if ch.is_whitespace()
            || ch == '('
            || ch == ')'
            || ch == '{'
            || ch == '}'
            || ch == ';'
        {
            tokens.push(Token::Constant(Constant::Int(number_str.parse().unwrap())));
            return;
        } else {
            panic!("lexing number, invalid character: {ch}");
        }
    }
}

fn identifier(ch: char, chars: &mut Peekable<impl Iterator<Item = char>>, tokens: &mut Vec<Token>) {
    let mut identifier_str = ch.to_string();

    while let Some(ch) = chars.peek() {
        let ch = *ch;

        if ch.is_alphanumeric() {
            identifier_str += ch.to_string().as_str();
            chars.next();
        } else if ch.is_whitespace()
            || ch == '('
            || ch == ')'
            || ch == '{'
            || ch == '}'
            || ch == ';'
        {
            let kw = keyword(identifier_str.as_str());

            if kw.is_some() {
                tokens.push(Token::Keyword(kw.unwrap()));
            } else {
                tokens.push(Token::Identifier(identifier_str));
            }

            return;
        } else {
            panic!("lexing identifier, invalid character: {ch}");
        }
    }
}

fn keyword(str: &str) -> Option<Keyword> {
    match str {
        "auto" => Some(Keyword::Auto),
        "break" => Some(Keyword::Break),
        "case" => Some(Keyword::Case),
        "char" => Some(Keyword::Char),
        "const" => Some(Keyword::Const),
        "continue" => Some(Keyword::Continue),
        "default" => Some(Keyword::Default),
        "do" => Some(Keyword::Do),
        "double" => Some(Keyword::Double),
        "else" => Some(Keyword::Else),
        "enum" => Some(Keyword::Enum),
        "extern" => Some(Keyword::Extern),
        "float" => Some(Keyword::Float),
        "for" => Some(Keyword::For),
        "goto" => Some(Keyword::Goto),
        "if" => Some(Keyword::If),
        "int" => Some(Keyword::Int),
        "long" => Some(Keyword::Long),
        "register" => Some(Keyword::Register),
        "return" => Some(Keyword::Return),
        "short" => Some(Keyword::Short),
        "signed" => Some(Keyword::Signed),
        "sizeof" => Some(Keyword::Sizeof),
        "static" => Some(Keyword::Static),
        "struct" => Some(Keyword::Struct),
        "switch" => Some(Keyword::Switch),
        "typedef" => Some(Keyword::Typedef),
        "union" => Some(Keyword::Union),
        "unsigned" => Some(Keyword::Unsigned),
        "void" => Some(Keyword::Void),
        "volatile" => Some(Keyword::Volatile),
        "while" => Some(Keyword::While),
        _ => None,
    }
}

fn string_literal(chars: &mut Peekable<impl Iterator<Item = char>>, tokens: &mut Vec<Token>) {
    let mut string_literal_str = String::new();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            tokens.push(Token::Constant(Constant::String(string_literal_str)));
            return;
        } else {
            string_literal_str += ch.to_string().as_str();
        }
    }

    panic!("lexing string literal: unterminated string literal");
}

fn comment(chars: &mut Peekable<impl Iterator<Item = char>>) {
    let next = *chars.peek().unwrap();

    if next == '/' {
        while let Some(ch) = chars.next() {
            if ch == '\n' {
                break;
            }
        }
    } else if next == '*' {
        chars.next();

        while let Some(ch) = chars.next() {
            if ch == '*' {
                if let Some(next) = chars.peek() {
                    if *next == '/' {
                        chars.next();
                        break;
                    }
                }
            }
        }
    } else {
        panic!("lexing comment: invalid character: {next}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_return_42() {
        let src = "int main() { return 42; }".to_string();
        let tokens = lex(src);
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Int),
                Token::Identifier("main".to_string()),
                Token::OpenParen,
                Token::CloseParen,
                Token::OpenCurly,
                Token::Keyword(Keyword::Return),
                Token::Constant(Constant::Int(42)),
                Token::Semicolon,
                Token::CloseCurly
            ]
        );
    }

    #[test]
    fn test_lex_single_line_comment() {
        let src = "// this is a comment\nreturn 1;".to_string();
        let tokens = lex(src);
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Return),
                Token::Constant(Constant::Int(1)),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_lex_multiline_comment() {
        let src = "/* this is\na multiline comment */return 2;".to_string();
        let tokens = lex(src);
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Return),
                Token::Constant(Constant::Int(2)),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_lex_return_string() {
        let src = r#"return "hello";"#.to_string();
        let tokens = lex(src);
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Return),
                Token::Constant(Constant::String("hello".to_string())),
                Token::Semicolon,
            ]
        );
    }
}
