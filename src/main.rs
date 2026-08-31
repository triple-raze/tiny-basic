mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let tokens = match lexer::tokenize("let x = 5") {
        Ok(t) => t,
        Err(msg) => {
            eprintln!("Syntax Error: {}", msg);
            std::process::exit(1)
        }
    };

    let a = parser::parse(tokens);
    println!("{:?}\n", a.unwrap());
}