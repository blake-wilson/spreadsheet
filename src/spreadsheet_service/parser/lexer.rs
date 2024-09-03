use super::parser::ASTNode;
use std::iter::Peekable;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Colon,
    Number(String),
    Op(char),
    Comma,
    Text(String),
    ID(String),
    Eof,
}

pub fn lex(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();

    println!("lexing: {}", input);

    while let Some(&c) = it.peek() {
        let t = match c {
            '0'..='9' => {
                let num = lex_number(&mut it);
                num
            }
            '(' => {
                it.next();
                Ok(Token::Op('('))
            }
            ')' => {
                it.next();
                Ok(Token::Op(')'))
            }
            '"' => {
                it.next();
                let mut str_val = "".to_string();
                while let Some(c) = it.next() {
                    if c == '"' {
                        break;
                    }
                    str_val.push(c);
                }
                Ok(Token::Text(str_val))
            }
            ':' => {
                it.next();
                Ok(Token::Colon)
            }
            ' ' => {
                it.next();
                continue;
            }
            ',' => {
                it.next();
                Ok(Token::Comma)
            }
            '*' | '+' | '-' | '/' => {
                it.next();
                Ok(Token::Op(c))
            }
            'A'..='z' => {
                let id = lex_id(&mut it);
                Ok(id)
            }
            _ => Err("unrecognized input"),
        };
        match t {
            Ok(t) => result.push(t),
            Err(e) => return Err(e),
        }
    }
    result.push(Token::Eof);
    println!("lexed tokens: {:?}", result);
    Ok(result)
}

fn lex_number<I>(input: &mut Peekable<I>) -> Result<Token, &'static str>
where
    I: Iterator<Item = char>,
{
    let mut val = String::new();
    // Take numbers until a non-number is encountered
    while let Some(Ok(_)) = input.peek().map(|c| c.to_string().parse::<u8>()) {
        val.push(input.next().unwrap());
    }
    Ok(Token::Number(val))
}

fn lex_id<I>(input: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = char>,
{
    let mut val = String::new();
    while let Some(&c) = input.peek() {
        if !is_id_char(c) {
            break;
        }
        val.push(c);
        input.next();
    }

    Token::ID(val.to_string())
}

pub fn is_id_char(c: char) -> bool {
    (c >= 'A' && c <= 'z') || (c >= '0' && c <= '9')
}
