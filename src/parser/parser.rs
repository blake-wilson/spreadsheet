use super::lexer::*;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Empty,
    Number(f64),
    UnaryExpr {
        op: Operator,
        child: Box<ASTNode>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Function {
        name: String,
        args: Vec<Box<ASTNode>>,
    },
}

pub fn parse(tokens: &mut Vec<Token>) -> ASTNode {
    if tokens.len() == 0 {
        return ASTNode::Empty;
    }
    let fst = tokens.remove(0);
    match fst.kind {
        TokenKind::Number => parse_number(&fst, tokens),
        TokenKind::ID => parse_function(&fst, tokens),
        x => {
            panic!("unrecognized token kind {:?}", x);
        }
    }
}

pub fn parse_number(curr: &Token, tokens: &mut Vec<Token>) -> ASTNode {
    let num_node = ASTNode::Number(curr.val.parse::<f64>().unwrap());
    if tokens.len() == 0 {
        return num_node;
    }
    let next = tokens.get(0).unwrap().clone();
    match next.kind {
        TokenKind::BinaryExpr => {
            tokens.remove(0);
            let rhs = parse(tokens);
            ASTNode::BinaryExpr {
                op: get_operator(&next.val),
                lhs: Box::new(num_node),
                rhs: Box::new(rhs),
            }
        }
        _ => {
            // Only binary expressions can follow numbers
            num_node
        }
    }
}

pub fn parse_function(curr: &Token, tokens: &mut Vec<Token>) -> ASTNode {
    let next = tokens.get(0).unwrap();

    if next.kind != TokenKind::LParen {
        panic!("unexpected token kind after function name: {:?}", next.kind);
    }
    tokens.remove(0);

    let mut args = Vec::new();
    while tokens.get(0).unwrap().kind != TokenKind::RParen {
        let arg = parse_function_argument(tokens);
        args.push(Box::new(arg));
    }

    ASTNode::Function {
        name: curr.val.clone(),
        args: args,
    }
}

pub fn parse_function_argument(tokens: &mut Vec<Token>) -> ASTNode {
    let arg = parse(tokens);
    let token = tokens.get(0).unwrap();
    if token.kind != TokenKind::RParen && token.kind != TokenKind::Comma {
        panic!("expected comma or right paren after function arg")
    }
    if token.kind == TokenKind::Comma {
        tokens.remove(0);
    }
    arg
}

pub fn get_operator(val: &str) -> Operator {
    match val {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        x => panic!("unrecognized operator {:?}", x),
    }
}
