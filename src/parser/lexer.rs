use std::iter::Peekable;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Colon,
    Number,
    BinaryExpr,
    Comma,
    Text,
    ID,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub val: String,
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
                Ok(Token {
                    kind: TokenKind::LParen,
                    val: "(".to_string(),
                })
            }
            ')' => {
                it.next();
                Ok(Token {
                    kind: TokenKind::RParen,
                    val: ")".to_string(),
                })
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
                Ok(Token {
                    kind: TokenKind::Text,
                    val: str_val,
                })
            }
            ':' => {
                it.next();
                Ok(Token {
                    kind: TokenKind::Colon,
                    val: ":".to_string(),
                })
            }
            ' ' => {
                it.next();
                continue;
            }
            ',' => {
                it.next();
                Ok(Token {
                    kind: TokenKind::Comma,
                    val: c.to_string(),
                })
            }
            '*' | '+' | '-' | '/' => {
                it.next();
                Ok(Token {
                    kind: TokenKind::BinaryExpr,
                    val: c.to_string(),
                })
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
    Ok(Token {
        kind: TokenKind::Number,
        val,
    })
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

    Token {
        kind: TokenKind::ID,
        val,
    }
}

pub fn is_id_char(c: char) -> bool {
    (c >= 'A' && c <= 'z') || (c >= '0' && c <= '9')
}
