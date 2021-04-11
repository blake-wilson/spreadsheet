mod parser;

fn main() {
    let mut tokens = parser::lex("CALL(10 * 3, CALLB(100, 200))");
    let tree = parser::parse(&mut tokens);
    println!("tree: {:?}", tree);
}
