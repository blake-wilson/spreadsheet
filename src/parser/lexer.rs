use std::iter::Peekable;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CellRef {
    pub col: i32,
    pub row: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    Colon,
    Number,
    BinaryExpr,
    Comma,
    Ref(CellRef),
    CellRange(CellRef, CellRef),
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
            '*' | '+' => {
                it.next();
                Ok(Token {
                    kind: TokenKind::BinaryExpr,
                    val: c.to_string(),
                })
            }
            // 'A'..='z' => lex_cellref(&mut it),
            'A'..='z' => {
                let id = lex_id(&mut it);
                Ok(id)
            }
            x => panic!("unrecognized input {:?}", x),
        };
        match t {
            Ok(t) => result.push(t),
            Err(e) => {
                panic!("error parsing expression: {}", e)
            }
        }
    }
    result
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
        if is_separator(c) {
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

pub fn is_separator(c: char) -> bool {
    c == '(' || c == ')' || c == ';' || c == ',' || c == ' '
}

// fn lex_cellref<I>(input: &mut Peekable<I>) -> Result<Token, &'static str>
// where
//     I: Iterator<Item = char>,
// {
//     let fst = lex_id(input)?;
//
//     if *input.peek().unwrap() != ':' {
//         return Ok(Token {
//             kind: TokenKind::Ref(fst),
//             val: "".to_string(),
//         });
//     }
//
//     // Expect a cell range
//     let snd = lex_id(input)?;
//     Ok(Token {
//         kind: TokenKind::CellRange(fst, snd),
//         val: "".to_string(),
//     })
// }

// fn lex_id<I>(input: &mut Peekable<I>) -> Result<CellRef, &'static str>
// where
//     I: Iterator<Item = char>,
// {
//     let mut col_specified = false;
//     let mut row_specified = false;
//
//     let mut col_str = "".to_string();
//     let mut row_str = "".to_string();
//
//     let mut val = String::new();
//     while let Some(&c) = input.peek() {
//         if !col_specified && (c < 'A' || c > 'z') {
//             return Err("expected a letter but did not find one for an ID");
//         }
//         if c >= 'A' && c <= 'z' {
//             if row_specified {
//                 return Err("row already specified but found column specifier");
//             }
//             col_specified = true;
//             col_str.push(c);
//         }
//         if c >= '0' && c <= '9' {
//             if !col_specified {
//                 return Err("col must be specified before row");
//             }
//             row_specified = true;
//             row_str.push(c);
//         }
//         val.push(c);
//         input.next();
//     }
//
//     Ok(CellRef {
//         row: row_str.parse::<i32>().unwrap(),
//         col: col_letters_to_num(&col_str),
//     })
// }
//
// fn col_letters_to_num(letters: &str) -> i32 {
//     let upper = letters.to_uppercase();
//     let mut total: i32 = 0;
//     for c in upper.chars() {
//         total += (c as i32) - 40;
//     }
//     total
// }
