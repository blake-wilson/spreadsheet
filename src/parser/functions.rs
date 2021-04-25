use super::parser::EvalResult;

pub fn evaluate_function(name: &str, args: Vec<EvalResult>) -> EvalResult {
    match name.to_uppercase().as_str() {
        "SUM" => sum(args),
        _ => EvalResult::NonNumeric("".to_owned()),
    }
}

pub fn sum(args: Vec<EvalResult>) -> EvalResult {
    let mut total = 0f64;
    for arg in args {
        match arg {
            EvalResult::Numeric(n) => total += n,
            _ => {} // treat non-numeric values as 0
        }
    }
    EvalResult::Numeric(total)
}
