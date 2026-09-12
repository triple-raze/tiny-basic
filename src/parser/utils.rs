use crate::token::{Punctuator, Token};

pub fn is_expr_token(token: &Token) -> bool {
    matches!(
        token,
        Token::Literal(_)
            | Token::Ident(_)
            | Token::MathOp(_)
            | Token::Eq
            | Token::InequalityOp(_)
            | Token::Punctuator(Punctuator::LParen)
            | Token::Punctuator(Punctuator::RParen)
    )
}
