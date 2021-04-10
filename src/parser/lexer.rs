use std::vec::Vec;
use std::iter::Peekable;

#[derive(Debug)]
pub enum TokenKind {
    LParen,
    RParen,
    Number,
    Comma,
    ID,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    val: String,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();

    while let Some(&c) = it.peek() {
        let t = match c {
            '0'..='9' => {
                it.next();
                lex_number(&mut it)
            },
            '(' => Token{
                kind: TokenKind::RParen,
                val: "(".to_string(),
                },
            ')' => Token{
                kind: TokenKind::LParen,
                val: ")".to_string(),
            },
            x => panic!("unrecognized input {:?}", x)
        };
        result.push(t);
    }
    result
}

fn lex_number<I>(input: &mut Peekable<I>) -> Token where I: Iterator<Item=char>  {
    let mut val = String::new();
    // Take numbers until a non-number is encountered
    while let Some(Ok(digit)) = input.peek().map(|c| c.to_string().parse::<char>()) {
        val.push(digit);
        input.next();
    }
    Token {
        kind: TokenKind::Number,
        val: val,
    }
}
