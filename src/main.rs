mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let tokens = match lexer::tokenize("10 let x = 5 20 if x = 5 then print \"abc\"") {
        Ok(t) => t,
        Err(msg) => {
            eprintln!("Syntax Error: {}", msg);
            std::process::exit(1)
        }
    };

    let a = parser::parse(tokens);
    println!("{:?}\n", a.unwrap());
}