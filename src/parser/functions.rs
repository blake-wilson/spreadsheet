use super::parser::EvalResult;

pub fn evaluate_function(name: &str, args: Vec<EvalResult>) -> EvalResult {
    match name.to_uppercase().as_str() {
        "SUM" => sum(args),
        "AVG" => avg(args),
        "MEDIAN" => median(args),
        "COUNT" => count(args),
        "ISEVEN" => is_even(args),
        "ISODD" => is_odd(args),
        _ => EvalResult::NonNumeric("".to_owned()),
    }
}

pub fn sum(args: Vec<EvalResult>) -> EvalResult {
    let numbers = numeric_values(args);
    let total = numbers.iter().fold(0f64, |acc, x| acc + x);

    EvalResult::Numeric(total)
}

pub fn avg(args: Vec<EvalResult>) -> EvalResult {
    let numbers = numeric_values(args);
    let total = numbers.iter().fold(0f64, |acc, x| acc + x);

    EvalResult::Numeric(total / numbers.len() as f64)
}

pub fn median(args: Vec<EvalResult>) -> EvalResult {
    let mut numbers = numeric_values(args);
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if numbers.len() == 0 {
        return EvalResult::NonNumeric("".to_string());
    }
    EvalResult::Numeric(numbers[numbers.len() / 2])
}

pub fn count(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::Numeric(args.iter().fold(0f64, |acc, _| acc + 1f64))
}

pub fn is_even(args: Vec<EvalResult>) -> EvalResult {
    if args.len() != 1 {
        return EvalResult::Error("#VALUE!".to_owned());
    }
    match args.get(0).unwrap() {
        EvalResult::Numeric(n) => {
            if n.fract() != 0.0 {
                return EvalResult::Error("#VALUE!".to_owned());
            }
            EvalResult::Bool((*n as i64) % 2 == 0)
        }
        _ => EvalResult::Error("#VALUE!".to_owned()),
    }
}

pub fn is_odd(args: Vec<EvalResult>) -> EvalResult {
    match is_even(args) {
        EvalResult::Bool(b) => EvalResult::Bool(!b),
        x => x,
    }
}

fn numeric_values(args: Vec<EvalResult>) -> Vec<f64> {
    args.into_iter()
        .filter_map(|x| {
            let num = match x {
                EvalResult::Numeric(n) => Some(n),
                _ => None,
            };
            num
        })
        .collect()
}
