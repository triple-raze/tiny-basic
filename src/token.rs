#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Then,
    Input,
    Print,
    Goto,
    Gosub,
    Return,
    End,
    Rem,
    Clear,
    List,
    Run,
}

#[derive(Debug, PartialEq)]
pub enum MathOp {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, PartialEq)]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    MathOp(MathOp),
    CompareOp(CompareOp),
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
            "REM" => Self::Keyword(Keyword::Rem),
            "CLEAR" => Self::Keyword(Keyword::Clear),
            "LIST" => Self::Keyword(Keyword::List),
            "RUN" => Self::Keyword(Keyword::Run),
            "+" => Self::MathOp(MathOp::Plus),
            "-" => Self::MathOp(MathOp::Minus),
            "*" => Self::MathOp(MathOp::Star),
            "/" => Self::MathOp(MathOp::Slash),
            "=" => Self::CompareOp(CompareOp::Eq),
            "<>" => Self::CompareOp(CompareOp::Ne),
            "<" => Self::CompareOp(CompareOp::Lt),
            "<=" => Self::CompareOp(CompareOp::Le),
            ">" => Self::CompareOp(CompareOp::Gt),
            ">=" => Self::CompareOp(CompareOp::Ge),
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
