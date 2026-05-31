use crate::token::{MathOp, Token};

mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let s = lexer::tokenize("x");
    println!(
        "{:?}",
        parser::parse_expr(&mut s.into_iter().peekable(), 0)
    )
}
