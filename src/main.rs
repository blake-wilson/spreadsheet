mod parser;

fn main() {
    println!("{:?}", parser::lex("((("));
}
