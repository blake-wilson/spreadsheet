use std::iter::Peekable;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Number,
    BinaryExpr,
    Comma,
    ID,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub val: String,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();

    while let Some(&c) = it.peek() {
        let t = match c {
            '0'..='9' => {
                let num = lex_number(&mut it);
                Some(num)
            }
            '(' => {
                it.next();
                Some(Token {
                    kind: TokenKind::LParen,
                    val: "(".to_string(),
                })
            }
            ')' => {
                it.next();
                Some(Token {
                    kind: TokenKind::RParen,
                    val: ")".to_string(),
                })
            }
            ' ' => {
                it.next();
                None
            }
            ',' => {
                it.next();
                Some(Token {
                    kind: TokenKind::Comma,
                    val: c.to_string(),
                })
            }
            '*' | '+' => {
                it.next();
                Some(Token {
                    kind: TokenKind::BinaryExpr,
                    val: c.to_string(),
                })
            }
            'A'..='z' => {
                let id = lex_id(&mut it);
                Some(id)
            }
            x => panic!("unrecognized input {:?}", x),
        };
        match t {
            Some(t) => result.push(t),
            _ => {}
        }
    }
    result
}

fn lex_number<I>(input: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = char>,
{
    let mut val = String::new();
    // Take numbers until a non-number is encountered
    while let Some(Ok(_)) = input.peek().map(|c| c.to_string().parse::<u8>()) {
        val.push(input.next().unwrap());
    }
    Token {
        kind: TokenKind::Number,
        val,
    }
}

fn lex_id<I>(input: &mut Peekable<I>) -> Token
where
    I: Iterator<Item = char>,
{
    let mut val = String::new();
    while let Some(&c) = input.peek() {
        if c < 'A' || c > 'z' {
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
