use std::iter::Peekable;
use std::str::Chars;

use crate::token::{Keyword, Literal, Token};

const OPERATOR_SYMBOLS: &[char] = &['+', '-', '*', '/', '=', '<', '>'];

fn prepare_source(source: &str) -> String {
    let mut in_string = false;

    let mut string = String::new();

    for ch in source.chars() {
        match ch {
            '"' => {
                in_string = !in_string;
                string.push('"');
            }
            _ => {
                // Doesnt change character if in users string
                if in_string {
                    string.push(ch);
                }
                // Makes character uppercase cuz basic is case insensitive and it removes inconsistency
                else {
                    string.push(ch.to_ascii_uppercase())
                }
            }
        }
    }

    string
}

// All tokenize_* functions should be used one character before the lexeme.
// For achieving this in tokenize() function, you need to peek at character instead of using next()
// and then, in match statement, you should use helper function on same iterator

fn tokenize_keyword_or_ident(chars: &mut Peekable<Chars>) -> Token {
    let mut string_repr = String::new();

    loop {
        match chars.peek() {
            Some(&ch) if ch.is_ascii_alphanumeric() => string_repr.push(chars.next().unwrap()),
            _ => {
                break;
            }
        }
    }

    Token::from_lexeme(string_repr.as_str())
}

fn tokenize_operator(chars: &mut Peekable<Chars>) -> Token {
    let mut string_repr = String::new();

    let first_ch = chars.next().unwrap();
    string_repr.push(first_ch);

    match chars.peek() {
        Some(&ch) if OPERATOR_SYMBOLS.contains(&ch) => {
            let second_ch = chars.next().unwrap();
            string_repr.push(second_ch)
        }
        _ => {}
    };

    let op = string_repr.as_str();

    if !Token::OPERATORS.contains(&op) {
        eprintln!("Unknown operator: {}", op);
        std::process::exit(1)
    };

    Token::from_lexeme(op)
}

fn tokenize_punctuator(chars: &mut Peekable<Chars>) -> Token {
    let ch = chars.next().unwrap();

    Token::from_char_lexeme(ch)
}

fn tokenize_str(chars: &mut Peekable<Chars>) -> Token {
    let mut string = String::new();
    // Skipping first quote
    chars.next();
    loop {
        match chars.next() {
            Some('"') => {
                break;
            }
            Some(ch) => {
                string.push(ch);
            }
            None => {
                eprintln!("Syntax error: unclosed string found.");
                std::process::exit(1)
            }
        }
    }
    Token::Literal(Literal::Str(string))
}
fn tokenize_num(chars: &mut Peekable<Chars>) -> Token {
    let mut string_repr = String::with_capacity(5); // Maximum of 5 characters in num
    loop {
        match chars.peek() {
            Some(ch) if ch.is_ascii_digit() => {
                string_repr.push(chars.next().unwrap());
            }
            _ => {
                break;
            }
        }
    }
    match string_repr.parse::<i16>() {
        Ok(value) => return Token::Literal(Literal::Num(value)),
        Err(_) => {
            eprintln!("Error: You should remove overflowing from literal values.");
            std::process::exit(1)
        }
    };
}

pub fn tokenize(raw_source: &str) -> Vec<Token> {
    let source = prepare_source(raw_source);
    let mut chars = source.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(&ch) = chars.peek() {
        // Function inside this match statement iterates through char
        let token = match ch {
            // Cases that does return token
            'a'..='z' | 'A'..='Z' => tokenize_keyword_or_ident(&mut chars),
            '"' => tokenize_str(&mut chars),
            '0'..='9' => tokenize_num(&mut chars),
            ch if Token::PUNCTUATORS.contains(&ch) => tokenize_punctuator(&mut chars),
            ch if OPERATOR_SYMBOLS.contains(&ch) => tokenize_operator(&mut chars),

            // Cases that doesnt returns token
            ch if ch.is_whitespace() => {
                // Skipping
                chars.next();
                continue;
            }
            _ => {
                eprintln!("Syntax Error: character {} found", ch);
                std::process::exit(1)
            }
        };

        // Removing commentaries from code
        if token == Token::Keyword(Keyword::Rem) {
            while let Some(ch) = chars.next() {
                if ch == '\n' {
                    break;
                }
            }
        };

        tokens.push(token);
    }

    return tokens;
}
