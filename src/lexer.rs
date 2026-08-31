use std::{borrow::Cow, iter::Peekable};
use std::str::Chars;

use crate::token::{Literal, Token};

/// Required for detecting operators
const OPERATOR_CHARS: &[char] = &['+', '-', '*', '/', '=', '<', '>'];

fn prepare_source(source: &str) -> String {
    let mut in_string = false;

    let mut new_source = String::with_capacity(source.len());

    for ch in source.chars() {
        match ch {
            '"' => {
                in_string = !in_string;
                new_source.push('"');
            }
            _ => {
                // Doesnt change character if in users string
                if in_string {
                    new_source.push(ch);
                }
                // Makes character uppercase cuz basic is case insensitive and it removes inconsistency
                else {
                    new_source.push(ch.to_ascii_uppercase())
                }
            }
        }
    }

    new_source
}

// All tokenize_* functions should be used one character before the lexeme.
// For achieving this in tokenize() function, you need to peek at character instead of using next()
// and then, in match statement, you should use helper function on same iterator

fn tokenize_keyword_or_ident(chars: &mut Peekable<Chars>) -> Token {
    let mut lexeme = String::new();

    while let Some(&ch) = chars.peek() {
        if !ch.is_ascii_alphanumeric() {
            break;
        }
        lexeme.push(chars.next().unwrap());
    }

    let token = Token::from_lexeme(lexeme.as_str());

    token
}

fn tokenize_operator<'a>(chars: &mut Peekable<Chars>) -> Result<Token, Cow<'a, str>> {
    let mut lexeme = String::new();

    let first_ch = match chars.next() {
        Some(c) => c,
        None => unreachable!("unexpected EOF using tokenize_operator")
    };
    lexeme.push(first_ch);

    if let Some(second_ch) = chars.peek() && OPERATOR_CHARS.contains(second_ch) {
        let second_ch = chars.next().unwrap();
        lexeme.push(second_ch)
    };

    let op = lexeme.as_str();

    if !Token::OPERATORS.contains(&op) {
        return Err(Cow::Owned(format!("unknown operator '{}'", op)));
    };

    Ok(Token::from_lexeme(op))
}

fn tokenize_punctuator(chars: &mut Peekable<Chars>) -> Token {
    let ch = match chars.next() {
        Some(c) => c,
        None => unreachable!("unexpected EOF using tokenize_punctuator")
    };

    Token::from_char_lexeme(ch)
}

fn tokenize_str<'a>(chars: &mut Peekable<Chars>) -> Result<Token, Cow<'a, str>> {
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
                return Err(Cow::Borrowed("unclosed string found"));
            }
        }
    }

    let token = Token::Literal(Literal::Str(string));
    Ok(token)
}

fn tokenize_num<'a>(chars: &mut Peekable<Chars>) -> Result<Token, Cow<'a, str>> {
    let mut lexeme = String::with_capacity(5); // Maximum of 5 characters in i16

    while let Some(ch) = chars.peek() {
        if !ch.is_ascii_digit() {
            break;
        }
        lexeme.push(chars.next().unwrap());
    }

    match lexeme.parse::<i16>() {
        Ok(value) => Ok(Token::Literal(Literal::Num(value))),
        Err(_) => {
            let err_msg = format!("integer overflow, {} > {}", lexeme, i16::MAX);
            Err(Cow::Owned(err_msg))
        }
    }
}

pub fn tokenize<'a>(source: &str) -> Result<Vec<Token>, Cow<'a, str>> {
    let source = prepare_source(source);
    let mut chars = source.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(&ch) = chars.peek() {
        // Function inside this match statement iterates through char
        let token = match ch {
            // Cases that does return token
            'a'..='z' | 'A'..='Z' => tokenize_keyword_or_ident(&mut chars),
            '"' => tokenize_str(&mut chars)?,
            '0'..='9' => tokenize_num(&mut chars)?,
            ch if Token::PUNCTUATORS.contains(&ch) => tokenize_punctuator(&mut chars),
            ch if OPERATOR_CHARS.contains(&ch) => tokenize_operator(&mut chars)?,

            // Cases that doesnt returns token
            ch if ch.is_whitespace() => {
                // Skipping
                chars.next();
                continue;
            }
            _ => {
                let err_msg = format!("unexpected character '{}'", ch);
                return Err(Cow::Owned(err_msg));
            }
        };

        // Removing commentaries from code
        if token == Token::Rem {
            while let Some(ch) = chars.next() {
                if ch == '\n' {
                    break;
                }
            };
        } else {
            tokens.push(token);
        }
    }

    return Ok(tokens);
}
