use crate::parser::utils::TokenIter;

use crate::ast::Expr;
use crate::token::{InequalityOp, MathOp, Punctuator, Token};

const DEFAULT_PRECEDENCE: u8 = 0;
const EQ_NE_PRECEDENCE: u8 = 1;
const INEQUALITY_PRECEDENCE: u8 = 2;
const PLUS_MINUS_PRECEDENCE: u8 = 3;
const STAR_SLASH_PRECEDENCE: u8 = 4;

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
        Token::InequalityOp(_) => INEQUALITY_PRECEDENCE,
        Token::Eq => EQ_NE_PRECEDENCE,
        _ => DEFAULT_PRECEDENCE,
    }
}

/// Parses token without left expression (unary operators, variables, literals)
fn nud(token: Token, iter: &mut TokenIter) -> Expr {
    match token {
        Token::MathOp(op) => {
            if op != MathOp::Minus {
                eprintln!("Syntax Error: unkown unary operator {:?}", op);
                std::process::exit(1)
            };

            let expr = parse_expr_with_precedence(iter, u8::MAX);
            Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            }
        }
        Token::Punctuator(Punctuator::LParen) => {
            let expr = parse_expr_with_precedence(iter, 0);
            match iter.next() {
                Some(Token::Punctuator(Punctuator::RParen)) => expr,
                token => {
                    eprintln!("Syntax Error: ')' excepted, found {:?}", token);
                    std::process::exit(1)
                }
            }
        }
        Token::Ident(name) => Expr::Variable { name },
        Token::Literal(literal) => Expr::Literal { literal },
        token => {
            eprintln!("Syntax Error: unknown token {:?}", token);
            std::process::exit(1)
        }
    }
}

/// Parses token with left expression (binary operators)
fn led(left: Expr, token: Token, iter: &mut TokenIter) -> Expr {
    match token {
        Token::MathOp(op) => {
            let right = parse_expr_with_precedence(iter, precedence_from_math_op(&op) + 1);
            Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Token::Eq => {
            let right = parse_expr_with_precedence(iter, EQ_NE_PRECEDENCE + 1);
            Expr::Eq {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Token::Ne => {
            let right = parse_expr_with_precedence(iter, EQ_NE_PRECEDENCE + 1);
            Expr::Ne {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Token::InequalityOp(op) => {
            let right = parse_expr_with_precedence(iter, INEQUALITY_PRECEDENCE + 1);
            match op {
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
            eprintln!("Unknown LED token {:?}", token);
            std::process::exit(1)
        }
    }
}

/// It is internal implementation and it should get iterator instead of vector
fn parse_expr_with_precedence(iter: &mut TokenIter, min_precedence: u8) -> Expr {
    let first = iter.next().unwrap();
    let mut left = nud(first, iter);

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
            Token::MathOp(_) | Token::Eq | Token::Ne | Token::InequalityOp(_) => {
                left = led(left, iter.next().unwrap(), iter)
            }
            _ => break,
        }
    }

    left
}

pub fn parse_expr(tokens: Vec<Token>) -> Expr {
    parse_expr_with_precedence(&mut tokens.into_iter().peekable(), 0)
}
