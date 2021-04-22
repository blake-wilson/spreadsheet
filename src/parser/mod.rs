pub use self::lexer::lex;
pub use self::parser::evaluate;
pub use self::parser::parse;

pub mod functions;
pub mod lexer;
pub mod parser;
mod test;
