use super::super::models::CellLocation;
use super::super::models::CellRange;
use super::super::models::EvalContext;
use super::functions::*;
use super::lexer::*;

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
            stop_row: self.row + 1,
            stop_col: self.col + 1,
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

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Bool(bool),
    Numeric(f64),
    NonNumeric(String),
    List(Vec<Box<EvalResult>>),
    Error(String),
}

pub fn parse(input: &str) -> Result<ASTNode, Error> {
    if input.starts_with('=') {
        let cell_value = input.strip_prefix('=').unwrap().to_string();
        let tokens = super::lexer::lex(&cell_value);
        match tokens {
            Ok(mut tks) => {
                return parse_internal(&mut tks);
            }
            Err(e) => {
                return Err(Error::new(e));
            }
        }
    }
    match input.parse::<f64>() {
        Ok(num) => Ok(ASTNode::Number(num)),
        _ => Ok(ASTNode::Text(input.to_owned())),
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
        ASTNode::BinaryExpr { op, lhs, rhs } => {
            println!("evaluating binary expression: {:?} {:?} {:?}", op, lhs, rhs);
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
            for arg in args {
                let eval_res = evaluate_internal(*arg, path, ctx);
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
            if path.contains(&cell_ref.loc()) {
                return EvalResult::Error("#CIRCULAR".to_owned());
            }
            path.push(cell_ref.loc());
            if !cell_ref.is_valid(ctx.num_rows(), ctx.num_cols()) {
                return EvalResult::Error("#REF".to_owned());
            }
            match ctx.get_cell(cell_ref.row, cell_ref.col) {
                Some(cell) => {
                    let parsed_val = parse(&cell.value).unwrap();
                    let res = evaluate_internal(parsed_val, path, ctx);
                    path.pop();
                    res
                }
                None => EvalResult::NonNumeric("".to_owned()),
            }
        }
        ASTNode::Range { start, mut stop } => {
            let mut results = vec![];
            if stop.is_unbounded() {
                stop.row = ctx.num_rows() - 1;
            }
            for i in start.row..stop.row + 1 {
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

pub fn parse_internal(tokens: &mut Vec<Token>) -> Result<ASTNode, Error> {
    if tokens.len() == 0 {
        return Ok(ASTNode::Empty);
    }
    print!("parse_internal: tokens are {:?}", tokens);
    let mut tree = ASTNode::Empty;
    while tokens.len() > 0 {
        println!("parsing tokens {:?}", tokens);
        let fst = tokens.get(0).unwrap().clone();
        if fst.kind != TokenKind::Comma {
            tokens.remove(0);
        }
        tree = match fst.kind {
            TokenKind::Number => parse_number(&tree, &fst, tokens),
            TokenKind::Text => Ok(ASTNode::Text(fst.val)),
            TokenKind::BinaryExpr => parse_binary_expr(&tree, &fst),
            TokenKind::LParen => parse_paren_expr(&tree, tokens),
            TokenKind::RParen => return Ok(tree),
            TokenKind::Comma => return Ok(tree),
            TokenKind::ID => match tokens.get(0) {
                Some(token) => match token.kind {
                    TokenKind::LParen => parse_function(&tree, &fst, tokens),
                    _ => parse_cell_ref_or_range(&fst, tokens),
                },
                None => parse_cell_ref_or_range(&fst, tokens),
            },
            x => Err(Error::new(&format!("unrecognized token kind {:?}", x))),
        }?;
        println!("tokens are now {:?}", tokens);
    }
    Ok(tree)
}

pub fn parse_number(parent: &ASTNode, curr: &Token, tokens: &[Token]) -> Result<ASTNode, Error> {
    println!("parsing number: {:?} {:?}", curr, tokens);
    match parent {
        ASTNode::Empty => Ok(ASTNode::BinaryExpr {
            op: Operator::Null,
            lhs: Box::new(ASTNode::Number(curr.val.parse::<f64>().unwrap())),
            rhs: Box::new(ASTNode::Empty),
        }),
        ASTNode::BinaryExpr { op, lhs, rhs } => match **lhs {
            ASTNode::Empty => Ok(ASTNode::BinaryExpr {
                op: op.clone(),
                lhs: Box::new(ASTNode::Number(curr.val.parse::<f64>().unwrap())),
                rhs: Box::new(ASTNode::Empty),
            }),
            _ => Ok(ASTNode::BinaryExpr {
                op: *op,
                lhs: lhs.clone(),
                rhs: Box::new(ASTNode::Number(curr.val.parse::<f64>().unwrap())),
            }),
        },
        x => Err(Error::new(&format!(
            "unexpected parent node type for number: {:?}",
            x
        ))),
    }
}

pub fn parse_paren_expr(tree: &ASTNode, tokens: &mut Vec<Token>) -> Result<ASTNode, Error> {
    let node = parse_internal(tokens)?;

    match tree {
        ASTNode::Empty => Ok(node.clone()),
        ASTNode::BinaryExpr { op, lhs, rhs } => Ok(match node {
            ASTNode::BinaryExpr {
                op,
                lhs: ref n_lhs,
                rhs: ref rhs,
            } => match op {
                Operator::Null => match **n_lhs {
                    ASTNode::Number(n) => ASTNode::BinaryExpr {
                        op: op.clone(),
                        lhs: lhs.clone(),
                        rhs: n_lhs.clone(),
                    },
                    _ => ASTNode::BinaryExpr {
                        op: op.clone(),
                        lhs: lhs.clone(),
                        rhs: Box::new(node.clone()),
                    },
                },
                _ => ASTNode::BinaryExpr {
                    op: op.clone(),
                    lhs: lhs.clone(),
                    rhs: Box::new(node.clone()),
                },
            },
            _ => ASTNode::BinaryExpr {
                op: op.clone(),
                lhs: Box::new(tree.clone()),
                rhs: Box::new(node.clone()),
            },
        }),
        _ => Err(Error::new(
            "unexpected node before parenthesized expression",
        )),
    }
}

fn parse_binary_expr(tree: &ASTNode, op: &Token) -> Result<ASTNode, Error> {
    let val = op.val.clone();
    match tree {
        ASTNode::Empty => Err(Error::new("No tree found for binary op")),
        ASTNode::BinaryExpr { op, lhs, rhs } => match op {
            Operator::Null => Ok(ASTNode::BinaryExpr {
                op: get_operator(&val)?,
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            }),
            // operator exists. Need to create a new parent for the tree. Assign the current tree
            // to this tree's left subtree
            _ => Ok(ASTNode::BinaryExpr {
                op: get_operator(&val)?,
                lhs: Box::new(tree.clone()),
                rhs: Box::new(ASTNode::Empty),
            }),
        },
        x => Err(Error::new(&format!(
            "unexpected tree node type for binary expression: {:?}",
            x
        ))),
    }

    // Ok(ASTNode::BinaryExpr {
    //     op: get_operator(&val)?,
    //     lhs: Box::new(lhs),
    //     rhs: Box::new(rhs),
    // })
}

pub fn parse_cell_ref_or_range(curr: &Token, tokens: &mut Vec<Token>) -> Result<ASTNode, Error> {
    let mut start = parse_cell_ref(curr)?;
    let next = tokens.get(0);

    let stop = match next {
        Some(t) => match t.kind {
            TokenKind::Colon => {
                tokens.remove(0);
                let stop = parse_cell_ref(tokens.get(0).unwrap())?;
                tokens.remove(0);
                start.row = 0;
                Some(stop)
            }
            _ => None,
        },
        _ => None,
    };
    let cell_ref = match stop {
        Some(s) => Ok(ASTNode::Range { start, stop: s }),
        None => {
            if !start.is_unbounded() {
                Ok(ASTNode::Ref(start))
            } else {
                Err(Error::new(&format!(
                    "cannot use unbounded reference without end column"
                )))
            }
        }
    }?;

    if tokens.len() == 0 {
        return Ok(cell_ref);
    }
    let next = tokens.get(0).unwrap().clone();
    match next.kind {
        TokenKind::BinaryExpr => return parse_binary_expr(&cell_ref.clone(), &next),
        _ => Ok(cell_ref),
    }
}

pub fn parse_function(
    tree: &ASTNode,
    curr: &Token,
    tokens: &mut Vec<Token>,
) -> Result<ASTNode, Error> {
    let next = tokens.get(0).unwrap();
    println!("parse_function tokens {:?}", tokens);

    if next.kind != TokenKind::LParen {
        return Err(Error::new(&format!(
            "unexpected token kind after function name: {:?}",
            next.kind
        )));
    }
    tokens.remove(0);

    let mut args = Vec::new();
    loop {
        args.push(Box::new(parse_function_argument(tokens)?));
        if tokens.get(0).is_none() {
            break;
        }
        let next = tokens.get(0).unwrap();
        if next.kind != TokenKind::Comma {
            break;
        }
        tokens.remove(0);
    }
    // if tokens.get(0).is_none() || tokens.get(0).unwrap().kind != TokenKind::RParen {
    //     return Err(Error::new(&format!(
    //         "No closing parentheses in function arguments",
    //     )));
    // }
    // tokens.remove(0);

    let funcNode = ASTNode::Function {
        name: curr.val.clone(),
        args,
    };
    match tree {
        ASTNode::Empty => Ok(ASTNode::BinaryExpr {
            op: Operator::Null,
            lhs: Box::new(funcNode),
            rhs: Box::new(ASTNode::Empty),
        }),
        ASTNode::BinaryExpr { op, lhs, rhs } => match **lhs {
            ASTNode::Empty => Ok(ASTNode::BinaryExpr {
                op: Operator::Null,
                lhs: Box::new(funcNode),
                rhs: Box::new(ASTNode::Empty),
            }),
            _ => Ok(ASTNode::BinaryExpr {
                op: Operator::Null,
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            }),
        },
        x => Err(Error::new(&format!(
            "unexpected parent node type for function node: {:?}",
            x
        ))),
    }
}

pub fn parse_function_argument(tokens: &mut Vec<Token>) -> Result<ASTNode, Error> {
    println!("parsing function argument\n\n");
    let arg = parse_internal(tokens)?;
    println!("function argument is {:?}", arg);
    Ok(arg)
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

fn parse_cell_ref(curr: &Token) -> Result<CellRef, Error> {
    let mut col_specified = false;
    let mut row_specified = false;

    let mut col_str = "".to_string();
    let mut row_str = "".to_string();

    let mut val = String::new();
    for c in curr.val.chars() {
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
