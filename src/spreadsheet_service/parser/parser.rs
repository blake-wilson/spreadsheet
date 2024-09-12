use super::super::models::CellLocation;
use super::super::models::CellRange;
use super::super::models::EvalContext;
use super::functions::*;
use super::lexer::*;
use std::cmp;

enum Precedence {
    PlusMinus = 2,
    MultDiv = 3,
    Exp = 4,
    Prefix = 5,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CellRef {
    pub col: i32,
    pub row: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    error_text: String,
}

impl Error {
    pub fn new(txt: &str) -> Error {
        Error {
            error_text: txt.to_owned(),
        }
    }
}

impl CellRef {
    fn is_unbounded(&self) -> bool {
        // '-1' is a magic number referring to an unbounded reference
        self.row == -1
    }

    fn is_valid(&self, max_row: i32, max_col: i32) -> bool {
        self.row < max_row && self.col < max_col
    }

    fn to_cell_range(&self) -> CellRange {
        CellRange {
            start_row: self.row,
            start_col: self.col,
            stop_row: self.row,
            stop_col: self.col,
        }
    }

    fn loc(&self) -> CellLocation {
        CellLocation {
            row: self.row,
            col: self.col,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Empty,
    Number(f64),
    Text(String),
    UnaryExpr {
        op: Operator,
        operand: Box<ASTNode>,
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
    ParseError(String),
}

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Bool(bool),
    Numeric(f64),
    NonNumeric(String),
    List(Vec<Box<EvalResult>>),
    Error(String),
}

pub fn parse(input: &str) -> ASTNode {
    if input.starts_with('=') {
        let cell_value = input.strip_prefix('=').unwrap().to_string();
        let tokens = super::lexer::lex(&cell_value);
        match tokens {
            Ok(mut tks) => {
                tks.reverse();
                return parse_internal(&mut tks);
            }
            Err(e) => {
                return ASTNode::ParseError(e.to_owned());
            }
        }
    }
    match input.parse::<f64>() {
        Ok(num) => ASTNode::Number(num),
        _ => ASTNode::Text(input.to_owned()),
    }
}

// evalute gets the display value for the provided AST
pub fn evaluate(n: ASTNode, ctx: &dyn EvalContext) -> String {
    let res = evaluate_internal(n, &mut vec![], ctx);
    match res {
        EvalResult::Numeric(n) => n.to_string(),
        EvalResult::Bool(b) => b.to_string(),
        EvalResult::NonNumeric(s) => s,
        EvalResult::List(_) => "".to_owned(),
        EvalResult::Error(msg) => msg.to_owned(),
    }
}

pub fn get_refs(n: &ASTNode) -> Vec<CellRange> {
    let mut refs = vec![];

    match n {
        ASTNode::BinaryExpr { op: _, lhs, rhs } => {
            refs.extend(get_refs(lhs));
            refs.extend(get_refs(rhs));
        }
        ASTNode::Function { name: _, args } => {
            for arg in args {
                refs.extend(get_refs(arg))
            }
        }
        ASTNode::Ref(cell_ref) => refs.push(cell_ref.to_cell_range()),
        ASTNode::Range { start, stop } => refs.push(CellRange {
            start_row: start.row,
            start_col: start.col,
            stop_row: stop.row,
            stop_col: stop.col,
        }),
        _ => (),
    }

    refs
}

fn evaluate_internal(
    n: ASTNode,
    path: &mut Vec<CellLocation>,
    ctx: &dyn EvalContext,
) -> EvalResult {
    match n {
        ASTNode::Empty => EvalResult::NonNumeric("".to_owned()),
        ASTNode::Number(n) => EvalResult::Numeric(n),
        ASTNode::Text(t) => EvalResult::NonNumeric(t),
        ASTNode::UnaryExpr { op, operand } => match evaluate_internal(*operand, path, ctx) {
            EvalResult::Error(e) => EvalResult::Error(e),
            v => match op {
                Operator::Subtract => sub(vec![EvalResult::Numeric(0f64), v]),
                _ => EvalResult::Error(format!("invalid unary operator {:?}", op)),
            },
        },
        ASTNode::BinaryExpr { op, lhs, rhs } => {
            match (
                evaluate_internal(*lhs, path, ctx),
                evaluate_internal(*rhs, path, ctx),
            ) {
                (EvalResult::Error(l), _) => EvalResult::Error(l),
                (_, EvalResult::Error(r)) => EvalResult::Error(r),
                (l, r) => match op {
                    Operator::Add => sum(vec![l, r]),
                    Operator::Subtract => sub(vec![l, r]),
                    Operator::Multiply => mult(vec![l, r]),
                    Operator::Divide => div(vec![l, r]),
                    Operator::Null => l,
                },
            }
        }
        ASTNode::Function { name, args } => {
            let mut evaluated_args = vec![];
            let mut eval_err: Option<String> = None;
            for arg in args {
                let eval_res = evaluate_internal(*arg, path, ctx);
                match eval_res {
                    EvalResult::List(results) => {
                        for res in results {
                            evaluated_args.push(*res);
                        }
                    }
                    EvalResult::Error(msg) => {
                        eval_err = Some(msg);
                    }
                    _ => evaluated_args.push(eval_res),
                }
            }
            if let Some(err_msg) = eval_err {
                EvalResult::Error(err_msg)
            } else {
                evaluate_function(&name, evaluated_args)
            }
        }
        ASTNode::Ref(cell_ref) => {
            if path.contains(&cell_ref.loc()) {
                return EvalResult::Error("#CIRCULAR".to_owned());
            }
            path.push(cell_ref.loc());
            if !cell_ref.is_valid(ctx.num_rows(), ctx.num_cols()) {
                return EvalResult::Error("#REF".to_owned());
            }
            match ctx.get_cell(cell_ref.row, cell_ref.col) {
                Some(cell) => {
                    let parsed_val = parse(&cell.value);
                    let res = evaluate_internal(parsed_val, path, ctx);
                    path.pop();
                    res
                }
                None => EvalResult::NonNumeric("".to_owned()),
            }
        }
        ASTNode::ParseError(e) => EvalResult::Error(e),
        ASTNode::Range { start, mut stop } => {
            let mut results = vec![];
            if stop.is_unbounded() {
                stop.row = ctx.num_rows() - 1;
            }
            for i in cmp::max(start.row, 0)..stop.row + 1 {
                for j in start.col..stop.col + 1 {
                    match ctx.get_cell(i, j) {
                        Some(_) => {
                            let res = evaluate_internal(
                                ASTNode::Ref(CellRef { row: i, col: j }),
                                path,
                                ctx,
                            );
                            results.push(Box::new(res));
                            path.pop();
                        }
                        None => {}
                    }
                }
            }
            EvalResult::List(results)
        }
    }
}

pub fn prefix_binding_power(op: char) -> (u8, u8) {
    (0, 0)
}

pub fn postfix_binding_power(op: char) -> Option<(u8, u8)> {
    Some((0, 0))
}

pub fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    match op {
        '+' | '-' => Some((1, 2)),
        '*' | '/' => Some((3, 4)),
        '^' => Some((5, 6)),
        _ => None,
    }
}

fn peek(tokens: &Vec<Token>) -> Token {
    tokens.last().unwrap_or(&Token::Eof).clone()
}

pub fn advance(tokens: &mut Vec<Token>) -> Token {
    tokens.pop().unwrap_or(Token::Eof)
}

pub fn parse_cell_or_function(id: String, tokens: &mut Vec<Token>) -> Result<ASTNode, Error> {
    let next = peek(tokens);
    match next {
        Token::Op('(') => {
            advance(tokens);
            // lhs is a function call
            let mut args = vec![];
            loop {
                println!("parse cell or func tokens are {:?}", tokens);
                let node = pratt_parse(tokens, 0)?;
                args.push(Box::new(node));
                match peek(tokens) {
                    Token::Comma => {
                        advance(tokens);
                        continue;
                    }
                    Token::Eof => break,
                    Token::Op(')') => {
                        advance(tokens);
                        break;
                    }
                    t => return Err(Error::new(&format!("unexpected token {:?}", t))),
                }
            }
            Ok(ASTNode::Function { name: id, args })
        }
        _ => {
            let left = parse_cell_ref(id)?;
            if let Token::Colon = peek(tokens) {
                advance(tokens);
                let right = match advance(tokens) {
                    Token::ID(ref_val) => parse_cell_ref(ref_val),
                    t => Err(Error::new(&format!(
                        "Could not parse value {:?} as cell reference",
                        t
                    ))),
                }?;
                Ok(ASTNode::Range {
                    start: left,
                    stop: right,
                })
            } else {
                Ok(ASTNode::Ref(left))
            }
        }
    }
}

pub fn pratt_parse(tokens: &mut Vec<Token>, mbp: u8) -> Result<ASTNode, Error> {
    println!("parsing tokens {:?}", tokens);
    let mut lhs = match advance(tokens) {
        Token::Op('(') => {
            let lhs = pratt_parse(tokens, 0);
            // assert_eq!(advance(tokens), Token::Op(')'));
            lhs
        }
        Token::Op(c) => {
            let op = get_operator(&c.to_string())?;
            let (_, r_bp) = prefix_binding_power(c);
            let rhs = pratt_parse(tokens, r_bp)?;
            Ok(ASTNode::UnaryExpr {
                op,
                operand: Box::new(rhs.clone()),
            })
        }
        Token::Number(txt) => match txt.parse::<f64>() {
            Ok(num) => Ok(ASTNode::Number(num)),
            Err(_) => Err(Error::new(&format!(
                "Could not parse value {:?} as number",
                txt
            ))),
        },
        Token::ID(id) => parse_cell_or_function(id, tokens),
        t => Err(Error::new(&format!("unexpected token {:?}", t))),
    }?;
    loop {
        let op = match peek(tokens) {
            Token::Eof | Token::Comma => break,
            Token::Op(op) => Ok(op),
            t => Err(Error::new(&format!("unexpected token {:?}", t))),
        }?;
        // if let Some((l_bp, _)) = postfix_binding_power(op) {
        //     if l_bp < mbp {
        //         break;
        //     }
        //     advance(tokens);
        // }
        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < mbp {
                break;
            }
            advance(tokens);
            lhs = match op {
                ':' => match lhs {
                    ASTNode::Ref(l) => {
                        advance(tokens);
                        let r_ref = pratt_parse(tokens, 0)?;
                        match r_ref {
                            ASTNode::Ref(r) => Ok(ASTNode::Range { start: l, stop: r }),
                            t => Err(Error::new(&format!(
                                "unexpected token in range reference {:?}",
                                t
                            ))),
                        }
                    }
                    t => Err(Error::new(&format!(
                        "unexpected token in range reference {:?}",
                        t
                    ))),
                }?,
                op => {
                    let rhs = pratt_parse(tokens, r_bp)?;
                    ASTNode::BinaryExpr {
                        op: get_operator(&op.to_string())?,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
            };
            continue;
        }
        break;
    }
    Ok(lhs)
}

pub fn parse_internal(tokens: &mut Vec<Token>) -> ASTNode {
    match pratt_parse(tokens, 0) {
        Ok(n) => n,
        Err(e) => ASTNode::ParseError(e.error_text.clone()),
    }
}

pub fn get_operator(val: &str) -> Result<Operator, Error> {
    match val {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Subtract),
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        x => Err(Error::new(
            &format!("unrecognized operator {:?}", x).to_owned(),
        )),
    }
}

fn parse_cell_ref(ref_val: String) -> Result<CellRef, Error> {
    let mut col_specified = false;
    let mut row_specified = false;

    let mut col_str = "".to_string();
    let mut row_str = "".to_string();

    let mut val = String::new();
    for c in ref_val.chars() {
        if !col_specified && (c < 'A' || c > 'z') {
            return Err(Error::new(
                &"expected a letter but did not find one for an ID".to_owned(),
            ));
        }
        if c >= 'A' && c <= 'z' {
            if row_specified {
                return Err(Error::new(
                    &"row already specified but found column specifier".to_owned(),
                ));
            }
            col_specified = true;
            col_str.push(c);
        }
        if c >= '0' && c <= '9' {
            if !col_specified {
                return Err(Error::new(&"col must be specified before row".to_owned()));
            }
            row_specified = true;
            row_str.push(c);
        }
        val.push(c);
    }

    let mut row = 0;
    if row_specified {
        row = match row_str.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(Error::new(&format!(
                "cannot parse row number from {}",
                row_str
            ))),
        }?;
    }
    if row < 0 {
        return Err(Error::new(&format!(
            "row number must be a non-negative integer but got {}",
            row
        )));
    }

    Ok(CellRef {
        col: col_letters_to_num(&col_str),
        // rows here are zero indexed, but one indexed in AST representation
        row: row - 1,
    })
}

fn col_letters_to_num(letters: &str) -> i32 {
    let upper = letters.to_uppercase();
    let mut total: i32 = 0;
    let mut mult = 1;

    for c in upper.chars().rev() {
        total += ((c as i32) - 64) * mult;
        mult *= 26;
    }
    total - 1
}
