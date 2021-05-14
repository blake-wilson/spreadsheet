pub use self::lexer::lex;
pub use self::parser::evaluate;
pub use self::parser::get_refs;
pub use self::parser::parse;
pub use self::parser::Error;

pub mod functions;
pub mod lexer;
pub mod parser;
mod test;
