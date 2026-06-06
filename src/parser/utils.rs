use std::iter::Peekable;
use std::vec::IntoIter;

use crate::token::{Punctuator, Token};

pub type TokenIter = Peekable<IntoIter<Token>>;

pub fn is_expr_token(token: &Token) -> bool {
    matches!(
        token,
        Token::Literal(_)
            | Token::Ident(_)
            | Token::MathOp(_)
            | Token::Eq
            | Token::Ne
            | Token::Punctuator(Punctuator::LParen)
            | Token::Punctuator(Punctuator::RParen)
    )
}
