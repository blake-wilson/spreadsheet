use super::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
}

#[derive(Debug, PartialEq)]
pub struct CellRef {
    col: i32,
    row: i32,
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
    Ref(CellRef),
    Range {
        start: CellRef,
        stop: CellRef,
    },
}

enum EvalResult {
    Numeric(f64),
    NonNumeric(String),
}

// evalute gets the display value for the provided AST
pub fn evaluate(n: ASTNode) -> String {
    let res = evaluate_internal(n);
    match res {
        EvalResult::Numeric(n) => n.to_string(),
        EvalResult::NonNumeric(s) => s,
    }
}

fn evaluate_internal(n: ASTNode) -> EvalResult {
    match n {
        ASTNode::Empty => EvalResult::NonNumeric("".to_owned()),
        ASTNode::Number(n) => EvalResult::Numeric(n),
        ASTNode::BinaryExpr { op, lhs, rhs } => {
            match (evaluate_internal(*lhs), evaluate_internal(*rhs)) {
                (EvalResult::Numeric(n1), EvalResult::Numeric(n2)) => match op {
                    Operator::Add => EvalResult::Numeric(n1 + n2),
                    Operator::Subtract => EvalResult::Numeric(n1 - n2),
                    Operator::Multiply => EvalResult::Numeric(n1 * n2),
                },
                _ => EvalResult::Numeric(0f64),
            }
        }
        _ => EvalResult::NonNumeric("".to_owned()),
    }
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<ASTNode, &'static str> {
    if tokens.len() == 0 {
        return Ok(ASTNode::Empty);
    }
    let fst = tokens.remove(0);
    match fst.kind {
        TokenKind::Number => parse_number(&fst, tokens),
        TokenKind::ID => match tokens.get(0) {
            Some(token) => match token.kind {
                TokenKind::LParen => Ok(parse_function(&fst, tokens)),
                TokenKind::Colon => parse_cell_ref_or_range(&fst, tokens),
                _ => {
                    panic!("unimplemented");
                }
            },
            _ => {
                panic!("unimplemented");
            }
        },
        x => {
            panic!("unrecognized token kind {:?}", x);
        }
    }
}

pub fn parse_number(curr: &Token, tokens: &mut Vec<Token>) -> Result<ASTNode, &'static str> {
    let num_node = ASTNode::Number(curr.val.parse::<f64>().unwrap());
    if tokens.len() == 0 {
        return Ok(num_node);
    }
    let next = tokens.get(0).unwrap().clone();
    match next.kind {
        TokenKind::BinaryExpr => {
            tokens.remove(0);
            let rhs = parse(tokens)?;
            Ok(ASTNode::BinaryExpr {
                op: get_operator(&next.val),
                lhs: Box::new(num_node),
                rhs: Box::new(rhs),
            })
        }
        _ => {
            // Only binary expressions can follow numbers
            Ok(num_node)
        }
    }
}

pub fn parse_cell_ref_or_range(
    curr: &Token,
    tokens: &mut Vec<Token>,
) -> Result<ASTNode, &'static str> {
    let start = parse_cell_ref(curr)?;
    let next = tokens.get(0).unwrap();

    if next.kind != TokenKind::Colon {
        return Ok(ASTNode::Ref(start));
    }
    tokens.remove(0);
    let stop = parse_cell_ref(tokens.get(0).unwrap())?;
    tokens.remove(0);
    Ok(ASTNode::Range { start, stop })
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
        args,
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
    arg.unwrap()
}

pub fn get_operator(val: &str) -> Operator {
    match val {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        x => panic!("unrecognized operator {:?}", x),
    }
}

fn parse_cell_ref(curr: &Token) -> Result<CellRef, &'static str> {
    let mut col_specified = false;
    let mut row_specified = false;

    let mut col_str = "".to_string();
    let mut row_str = "".to_string();

    let mut val = String::new();
    for c in curr.val.chars() {
        if !col_specified && (c < 'A' || c > 'z') {
            return Err("expected a letter but did not find one for an ID");
        }
        if c >= 'A' && c <= 'z' {
            if row_specified {
                return Err("row already specified but found column specifier");
            }
            col_specified = true;
            col_str.push(c);
        }
        if c >= '0' && c <= '9' {
            if !col_specified {
                return Err("col must be specified before row");
            }
            row_specified = true;
            row_str.push(c);
        }
        val.push(c);
    }

    Ok(CellRef {
        row: row_str.parse::<i32>().unwrap(),
        col: col_letters_to_num(&col_str),
    })
}

fn col_letters_to_num(letters: &str) -> i32 {
    let upper = letters.to_uppercase();
    let mut total: i32 = 0;
    for c in upper.chars() {
        total += (c as i32) - 40;
    }
    total
}
