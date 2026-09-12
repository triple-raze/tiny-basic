use strum_macros::Display;
use std::vec::IntoIter;
use std::iter::Peekable;

#[derive(Display, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    Let,
    If,
    Then,
    Else,
    Input,
    Print,
    Goto,
    Gosub,
    Return,
    End,
}

#[derive(Display, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum MathOp {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Display, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum InequalityOp {
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Display, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Punctuator {
    Comma,
    Colon,
    Semicolon,

    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(i16),
}

#[derive(Display, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Token {
    Keyword(Keyword),
    Rem,  // Its a keyword, but its ignored after lexing. Seperating these helps keep code cleaner
    MathOp(MathOp),
    Eq,  // Eq has two meanings depending on context (comprasion and assignment)
    InequalityOp(InequalityOp),
    Punctuator(Punctuator),
    Literal(Literal),
    Ident(String),
}

impl Token {
    pub const OPERATORS: &[&str] = &["+", "-", "*", "/", "=", "<>", "<", "<=", ">", ">="];
    pub const PUNCTUATORS: &[char] = &[',', ':', ';', '(', ')'];

    pub fn from_lexeme(s: &str) -> Self {
        match s {
            "LET" => Self::Keyword(Keyword::Let),
            "IF" => Self::Keyword(Keyword::If),
            "THEN" => Self::Keyword(Keyword::Then),
            "INPUT" => Self::Keyword(Keyword::Input),
            "PRINT" => Self::Keyword(Keyword::Print),
            "GOTO" => Self::Keyword(Keyword::Goto),
            "GOSUB" => Self::Keyword(Keyword::Gosub),
            "RETURN" => Self::Keyword(Keyword::Return),
            "END" => Self::Keyword(Keyword::End),
            "REM" => Self::Rem,
            // TODO: add these in (currently non-existent) interactive mode
            // "CLEAR" => Self::Keyword(Keyword::Clear),
            // "LIST" => Self::Keyword(Keyword::List),
            // "RUN" => Self::Keyword(Keyword::Run),
            "+" => Self::MathOp(MathOp::Plus),
            "-" => Self::MathOp(MathOp::Minus),
            "*" => Self::MathOp(MathOp::Star),
            "/" => Self::MathOp(MathOp::Slash),
            "=" => Self::Eq,
            "<>" => Self::InequalityOp(InequalityOp::Ne),
            "<" => Self::InequalityOp(InequalityOp::Lt),
            "<=" => Self::InequalityOp(InequalityOp::Le),
            ">" => Self::InequalityOp(InequalityOp::Gt),
            ">=" => Self::InequalityOp(InequalityOp::Ge),
            "," => Self::Punctuator(Punctuator::Comma),
            ":" => Self::Punctuator(Punctuator::Colon),
            ";" => Self::Punctuator(Punctuator::Semicolon),
            "(" => Self::Punctuator(Punctuator::LParen),
            ")" => Self::Punctuator(Punctuator::RParen),
            _ => match s.parse::<i16>() {
                Ok(num) => Self::Literal(Literal::Num(num)),
                Err(_) => Self::Ident(s.to_string()),
            },
        }
    }

    pub fn from_char_lexeme(ch: char) -> Self {
        let mut buffer = [0; 4];
        let s = ch.encode_utf8(&mut buffer);

        Token::from_lexeme(s)
    }
}

pub type TokenIter = Peekable<IntoIter<Token>>;