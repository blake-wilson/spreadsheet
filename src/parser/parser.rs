use super::super::models::CellRange;
use super::super::models::EvalContext;
use super::functions::*;
use super::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub struct CellRef {
    pub col: i32,
    pub row: i32,
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
            stop_row: self.row + 1,
            stop_col: self.col + 1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Empty,
    Number(f64),
    Text(String),
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

pub enum EvalResult {
    Numeric(f64),
    NonNumeric(String),
    List(Vec<Box<EvalResult>>),
    Error(String),
}

pub fn parse(input: &str) -> Result<ASTNode, String> {
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
                    Operator::Divide => EvalResult::Numeric(n1 / n2),
                },
                (EvalResult::Error(l), _) => EvalResult::Error(l),
                (_, EvalResult::Error(r)) => EvalResult::Error(r),
                _ => EvalResult::Numeric(0f64),
            }
        }
        ASTNode::Function { name, args } => {
            println!("evaluating function {}", name);
            let mut evaluated_args = vec![];
            for arg in args {
                let eval_res = evaluate_internal(*arg, ctx);
                match eval_res {
                    EvalResult::List(results) => {
                        for res in results {
                            evaluated_args.push(*res);
                        }
                    }
                    _ => evaluated_args.push(eval_res),
                }
            }
            evaluate_function(&name, evaluated_args)
        }
        ASTNode::Ref(cell_ref) => {
            if !cell_ref.is_valid(ctx.num_rows(), ctx.num_cols()) {
                return EvalResult::Error("#REF".to_owned());
            }
            let cell_value = ctx.get_cell(cell_ref.row, cell_ref.col).value;
            let parsed_val = parse(&cell_value).unwrap();
            evaluate_internal(parsed_val, ctx)
        }
        ASTNode::Range { start, mut stop } => {
            println!("evaluate range: {:?}, {:?}", start, stop);
            let mut results = vec![];
            if stop.is_unbounded() {
                stop.row = ctx.num_rows() - 1;
            }
            for i in start.row..stop.row + 1 {
                for j in start.col..stop.col + 1 {
                    let res = evaluate_internal(ASTNode::Ref(CellRef { row: i, col: j }), ctx);
                    results.push(Box::new(res));
                }
            }
            EvalResult::List(results)
        }
    }
}

pub fn parse_internal(tokens: &mut Vec<Token>) -> Result<ASTNode, String> {
    if tokens.len() == 0 {
        return Ok(ASTNode::Empty);
    }
    let fst = tokens.remove(0);
    match fst.kind {
        TokenKind::Number => parse_number(&fst, tokens),
        TokenKind::Text => Ok(ASTNode::Text(fst.val)),
        TokenKind::ID => match tokens.get(0) {
            Some(token) => match token.kind {
                TokenKind::LParen => parse_function(&fst, tokens),
                _ => parse_cell_ref_or_range(&fst, tokens),
            },
            None => parse_cell_ref_or_range(&fst, tokens),
        },
        x => Err(format!("unrecognized token kind {:?}", x)),
    }
}

pub fn parse_number(curr: &Token, tokens: &mut Vec<Token>) -> Result<ASTNode, String> {
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
                op: get_operator(&next.val)?,
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

pub fn parse_cell_ref_or_range(curr: &Token, tokens: &mut Vec<Token>) -> Result<ASTNode, String> {
    let mut start = parse_cell_ref(curr)?;
    let next = tokens.get(0);

    println!("cell ref or range next token {:?}", next);

    let cell_ref = match next {
        Some(t) => match t.kind {
            TokenKind::Colon => {
                tokens.remove(0);
                let mut stop = parse_cell_ref(tokens.get(0).unwrap())?;
                tokens.remove(0);
                start.row = 0;
                stop.row = -1;
                ASTNode::Range { start, stop }
            }
            _ => ASTNode::Ref(start),
        },
        None => ASTNode::Ref(start),
    };

    let next = tokens.get(0);
    match next {
        Some(t) => match t.kind {
            TokenKind::BinaryExpr => {
                let val = t.val.clone();
                tokens.remove(0);
                let rhs = parse_internal(tokens)?;
                Ok(ASTNode::BinaryExpr {
                    op: get_operator(&val)?,
                    lhs: Box::new(cell_ref),
                    rhs: Box::new(rhs),
                })
            }
            _ => Ok(cell_ref),
        },
        None => Ok(cell_ref),
    }
}

pub fn parse_function(curr: &Token, tokens: &mut Vec<Token>) -> Result<ASTNode, String> {
    let next = tokens.get(0).unwrap();

    if next.kind != TokenKind::LParen {
        return Err(format!(
            "unexpected token kind after function name: {:?}",
            next.kind
        ));
    }
    tokens.remove(0);

    let mut args = Vec::new();
    while tokens.get(0).unwrap().kind != TokenKind::RParen {
        let arg = parse_function_argument(tokens)?;
        args.push(Box::new(arg));
    }

    Ok(ASTNode::Function {
        name: curr.val.clone(),
        args,
    })
}

pub fn parse_function_argument(tokens: &mut Vec<Token>) -> Result<ASTNode, String> {
    let arg = parse_internal(tokens);
    let token = tokens.get(0).unwrap();
    if token.kind != TokenKind::RParen && token.kind != TokenKind::Comma {
        return Err("expected comma or right paren after function arg".to_owned());
    }
    if token.kind == TokenKind::Comma {
        tokens.remove(0);
    }
    Ok(arg.unwrap())
}

pub fn get_operator(val: &str) -> Result<Operator, String> {
    match val {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Subtract),
        "*" => Ok(Operator::Multiply),
        "/" => Ok(Operator::Divide),
        x => Err(format!("unrecognized operator {:?}", x).to_owned()),
    }
}

fn parse_cell_ref(curr: &Token) -> Result<CellRef, String> {
    let mut col_specified = false;
    let mut row_specified = false;
    let mut colon_specified = false;

    let mut col_str = "".to_string();
    let mut row_str = "".to_string();

    let mut val = String::new();
    println!("{:?}", curr.val.chars());
    for c in curr.val.chars() {
        if !col_specified && (c < 'A' || c > 'z') {
            return Err("expected a letter but did not find one for an ID".to_owned());
        }
        if c >= 'A' && c <= 'z' {
            if row_specified {
                return Err("row already specified but found column specifier".to_owned());
            }
            col_specified = true;
            col_str.push(c);
        }
        if c >= '0' && c <= '9' {
            if !col_specified {
                return Err("col must be specified before row".to_owned());
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
            Err(_) => Err(format!("cannot parse row number from {}", row_str)),
        }?;
    }
    if row < 0 {
        return Err(format!(
            "row number must be a non-negative integer but got {}",
            row
        ));
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

    for c in upper.chars() {
        total += ((c as i32) - 65) * mult;
        mult += 1;
    }
    total
}
