use super::parser::EvalResult;

pub fn evaluate_function(name: &str, args: Vec<EvalResult>) -> EvalResult {
    match name.to_uppercase().as_str() {
        "SUM" => sum(args),
        "AVG" => avg(args),
        // "MEDIAN" => median(args),
        _ => EvalResult::NonNumeric("".to_owned()),
    }
}

pub fn sum(args: Vec<EvalResult>) -> EvalResult {
    let total = args.iter().fold(0f64, |acc, x| match x {
        &EvalResult::Numeric(n) => acc + n,
        _ => acc,
    });

    EvalResult::Numeric(total)
}

pub fn avg(args: Vec<EvalResult>) -> EvalResult {
    let mut num_numeric = 0;
    let total = args.iter().fold(0f64, |acc, x| match x {
        &EvalResult::Numeric(n) => {
            num_numeric += 1;
            acc + n
        }
        _ => acc,
    });

    EvalResult::Numeric(total / num_numeric as f64)
}

// pub fn median(args: Vec<EvalResult>) -> EvalResult {
//     let mut num_numeric = 0;
//     let total = args.iter().fold(0f64, |acc, x| match x {
//         &EvalResult::Numeric(n) => {
//             num_numeric += 1;
//             acc + n
//         }
//     });
//
//     EvalResult::Numeric(total / num_numeric as f64)
// }
