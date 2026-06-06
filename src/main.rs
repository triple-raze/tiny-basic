mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let t = lexer::tokenize("if 10 = 10 then print 5 let zalupa = 10");
    println!("{:?}", t);

    let a = parser::parse(t);
    println!("{:?}", a)
}
