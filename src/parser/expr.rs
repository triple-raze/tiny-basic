use std::borrow::Cow;

use crate::parser::utils::TokenIter;

use crate::ast::Expr;
use crate::token::{InequalityOp, MathOp, Punctuator, Token};

const DEFAULT_PRECEDENCE: u8 = 0;
const COMPARSION_PRECEDENCE: u8 = 1;
const PLUS_MINUS_PRECEDENCE: u8 = 2;
const STAR_SLASH_PRECEDENCE: u8 = 3;

// precedence means priority of tokens in math expression
const fn precedence_from_math_op(op: &MathOp) -> u8 {
    match op {
        MathOp::Plus | MathOp::Minus => PLUS_MINUS_PRECEDENCE,
        MathOp::Star | MathOp::Slash => STAR_SLASH_PRECEDENCE,
    }
}

// Literals and variables should have precedence=0
const fn precedence_from_token(token: &Token) -> u8 {
    match token {
        Token::MathOp(op) => precedence_from_math_op(op),
        Token::InequalityOp(_) => COMPARSION_PRECEDENCE,
        Token::Eq => COMPARSION_PRECEDENCE,
        _ => DEFAULT_PRECEDENCE,
    }
}

/// Parses token without left expression (unary operators, variables, literals)
fn nud<'a>(token: Token, iter: &mut TokenIter) -> Result<Expr, Cow<'a, str>> {
    match token {
        Token::MathOp(op) => {
            if op != MathOp::Minus {
                let err_msg = format!("unkown unary operator {:?}", op);
                return Err(Cow::Owned(err_msg));
            };

            let expr = parse_expr_with_precedence(iter, u8::MAX)?;
            Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            })
        }
        Token::Punctuator(Punctuator::LParen) => {
            let expr = parse_expr_with_precedence(iter, 0)?;
            match iter.next() {
                Some(Token::Punctuator(Punctuator::RParen)) => Ok(expr),
                token => {
                    let err_msg = format!("')' excepted, found {:?}", token);
                    Err(Cow::Owned(err_msg))
                }
            }
        }
        Token::Ident(name) => Ok(Expr::Variable { name }),
        Token::Literal(literal) => Ok(Expr::Literal { literal }),
        token => {
            let err_msg = format!("Syntax Error: unknown token {:?}", token);
            Err(Cow::Owned(err_msg))
        }
    }
}

/// Parses token with left expression (binary operators)
fn led<'a>(left: Expr, token: Token, iter: &mut TokenIter) -> Result<Expr, Cow<'a, str>> {
    let expr = match token {
        Token::MathOp(op) => {
            let right = parse_expr_with_precedence(iter, precedence_from_math_op(&op) + 1)?;
            Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Token::Eq => {
            let right = parse_expr_with_precedence(iter, COMPARSION_PRECEDENCE + 1)?;
            Expr::Eq {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Token::InequalityOp(op) => {
            let right = parse_expr_with_precedence(iter, COMPARSION_PRECEDENCE + 1)?;
            match op {
                InequalityOp::Ne => Expr::Ne {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                InequalityOp::Lt => Expr::Lt {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                InequalityOp::Le => Expr::Le {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                InequalityOp::Gt => Expr::Gt {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                InequalityOp::Ge => Expr::Ge {
                    left: Box::new(left),
                    right: Box::new(right),
                },
            }
        }
        _ => {
            return Err(Cow::Owned(format!("unexpected token '{}'", token)))
        }
    };
    Ok(expr)
}

/// It is internal implementation and it should get iterator instead of vector
fn parse_expr_with_precedence<'a>(iter: &mut TokenIter, min_precedence: u8) -> Result<Expr, Cow<'a, str>> {
    let first = iter.next().unwrap();
    let mut left = nud(first, iter)?;

    while let Some(token) = iter.peek() {
        if token == &Token::Punctuator(Punctuator::LParen)
            || token == &Token::Punctuator(Punctuator::RParen)
        {
            break;
        };

        if precedence_from_token(token) < min_precedence {
            break;
        }

        let next = iter.peek().unwrap();
        match next {
            Token::MathOp(_) | Token::Eq | Token::InequalityOp(_) => {
                left = led(left, iter.next().unwrap(), iter)?
            }
            _ => break,
        }
    }

    Ok(left)
}

pub fn parse_expr<'a>(tokens: Vec<Token>) -> Result<Expr, Cow<'a, str>> {
    parse_expr_with_precedence(&mut tokens.into_iter().peekable(), 0)
}
