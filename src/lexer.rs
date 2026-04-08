use std::{fs, iter::Peekable};

#[derive(Debug, PartialEq)]
pub(crate) enum Constant {
    Int(u64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Keyword {
    Int,
    Return,
    Void,
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
}

pub(crate) fn lex_file(filename: &str) -> Vec<Token> {
    let entire_file = fs::read_to_string(filename).unwrap();

    lex(entire_file)
}

fn lex(s: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_numeric() {
            number(ch, &mut chars, &mut tokens);
        } else if ch.is_alphabetic() {
            identifier(ch, &mut chars, &mut tokens);
        } else if ch == '"' {
            string_literal(&mut chars, &mut tokens);
        } else if ch == '{' {
            tokens.push(Token::OpenCurly);
        } else if ch == '}' {
            tokens.push(Token::CloseCurly);
        } else if ch == '(' {
            tokens.push(Token::OpenParen);
        } else if ch == ')' {
            tokens.push(Token::CloseParen);
        } else if ch == ';' {
            tokens.push(Token::Semicolon);
        } else if ch.is_whitespace() {
            continue;
        } else if ch == '/' {
            comment(&mut chars);
        } else {
            panic!("lexing: invalid character: {ch}");
        }
    }

    tokens
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
        "int" => Some(Keyword::Int),
        "return" => Some(Keyword::Return),
        "void" => Some(Keyword::Void),
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
