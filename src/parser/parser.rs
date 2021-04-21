use super::super::models::EvalContext;
use super::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
}

#[derive(Debug, PartialEq)]
pub struct CellRef {
    pub col: i32,
    pub row: i32,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Empty,
    Number(f64),
    Text(String),
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

pub fn parse(input: &str) -> Result<ASTNode, &'static str> {
    if input.starts_with('=') {
        let cell_value = input.strip_prefix('=').unwrap().to_string();
        let mut tokens = super::lexer::lex(&cell_value);
        return parse_internal(&mut tokens);
    }
    match input.parse::<f64>() {
        Ok(num) => Ok(ASTNode::Number(num)),
        _ => Ok(ASTNode::Text(input.to_owned())),
    }
}

// evalute gets the display value for the provided AST
pub fn evaluate(n: ASTNode, ctx: &dyn EvalContext) -> String {
    let res = evaluate_internal(n, ctx);
    match res {
        EvalResult::Numeric(n) => n.to_string(),
        EvalResult::NonNumeric(s) => s,
    }
}

fn evaluate_internal(n: ASTNode, ctx: &dyn EvalContext) -> EvalResult {
    match n {
        ASTNode::Empty => EvalResult::NonNumeric("".to_owned()),
        ASTNode::Number(n) => EvalResult::Numeric(n),
        ASTNode::Text(t) => EvalResult::NonNumeric(t),
        ASTNode::BinaryExpr { op, lhs, rhs } => {
            match (evaluate_internal(*lhs, ctx), evaluate_internal(*rhs, ctx)) {
                (EvalResult::Numeric(n1), EvalResult::Numeric(n2)) => match op {
                    Operator::Add => EvalResult::Numeric(n1 + n2),
                    Operator::Subtract => EvalResult::Numeric(n1 - n2),
                    Operator::Multiply => EvalResult::Numeric(n1 * n2),
                },
                _ => EvalResult::Numeric(0f64),
            }
        }
        ASTNode::Ref(cell_ref) => {
            println!("evaluating ref {:?}", cell_ref);
            let mut cell_value = ctx.get_cell(cell_ref.row, cell_ref.col).value;
            if cell_value.starts_with('=') {
                cell_value = cell_value.strip_prefix('=').unwrap().to_string();
            }
            let mut tokens = super::lexer::lex(&cell_value);
            let parsed_val = parse_internal(&mut tokens).unwrap();
            evaluate_internal(parsed_val, ctx)
        }
        _ => EvalResult::NonNumeric("".to_owned()),
    }
}

fn parse_internal(tokens: &mut Vec<Token>) -> Result<ASTNode, &'static str> {
    if tokens.len() == 0 {
        return Ok(ASTNode::Empty);
    }
    let fst = tokens.remove(0);
    match fst.kind {
        TokenKind::Number => parse_number(&fst, tokens),
        TokenKind::Text => Ok(ASTNode::Text(fst.val)),
        TokenKind::ID => match tokens.get(0) {
            Some(token) => match token.kind {
                TokenKind::LParen => Ok(parse_function(&fst, tokens)),
                _ => parse_cell_ref_or_range(&fst, tokens),
            },
            None => parse_cell_ref_or_range(&fst, tokens),
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
            let rhs = parse_internal(tokens)?;
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
    let next = tokens.get(0);

    let cell_ref = match next {
        Some(t) => match t.kind {
            TokenKind::Colon => {
                tokens.remove(0);
                let stop = parse_cell_ref(tokens.get(0).unwrap())?;
                tokens.remove(0);
                Ok(ASTNode::Range { start, stop })
            }
            _ => Ok(ASTNode::Ref(start)),
        },
        None => Ok(ASTNode::Ref(start)),
    }?;

    let next = tokens.get(0);
    match next {
        Some(t) => match t.kind {
            TokenKind::BinaryExpr => {
                let val = t.val.clone();
                tokens.remove(0);
                let rhs = parse_internal(tokens)?;
                Ok(ASTNode::BinaryExpr {
                    op: get_operator(&val),
                    lhs: Box::new(cell_ref),
                    rhs: Box::new(rhs),
                })
            }
            _ => Ok(cell_ref),
        },
        None => Ok(cell_ref),
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
        args,
    }
}

pub fn parse_function_argument(tokens: &mut Vec<Token>) -> ASTNode {
    let arg = parse_internal(tokens);
    let token = tokens.get(0).unwrap();
    println!("token: {:?}", token);
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
    println!("{:?}", curr.val.chars());
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

    println!("row number: {}", row_str.parse::<i32>().unwrap() - 1);
    Ok(CellRef {
        // rows here are zero indexed, but one indexed in AST representation
        row: row_str.parse::<i32>().unwrap() - 1,
        col: col_letters_to_num(&col_str),
    })
}

fn col_letters_to_num(letters: &str) -> i32 {
    let upper = letters.to_uppercase();
    let mut total: i32 = 0;
    let mut mult = 1;
    for c in upper.chars() {
        total += ((c as i32) - 65) * mult;
        mult += 1;
    }
    println!("col number: {}", total);
    total
}
