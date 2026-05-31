use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{Expr, Stmt};
use crate::token::{Keyword, Literal, MathOp, Punctuator, Token};

type TokenIter = Peekable<IntoIter<Token>>;
type ExprIter = Peekable<IntoIter<Expr>>;

// Precedence means priority of tokens in math expression
const fn precedence_from_op(op: &MathOp) -> u8 {
    match op {
        MathOp::Plus | MathOp::Minus => 1,
        MathOp::Star | MathOp::Slash => 2,
    }
}

// Literals and variables should have precedence=0
const fn precedence(token: &Token) -> u8 {
    match token {
        Token::MathOp(op) => precedence_from_op(op),
        _ => 0,
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

            let expr = parse_expr(iter, u8::MAX);
            Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            }
        },
        Token::Punctuator(Punctuator::LParen) => {
            let expr = parse_expr(iter, 0);
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
        },
    }
}

/// Parses token with left expression (binary operators)
fn led(left: Expr, token: Token, iter: &mut TokenIter) -> Expr {
    match token {
        Token::MathOp(op) => {
            let right = parse_expr(iter, precedence_from_op(&op) + 1);
            Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        _ => {
            eprintln!("Error: Unknown LED token {:?}", token);
            std::process::exit(1)
        }
    }
}

pub fn parse_expr(tokens: &mut TokenIter, min_precedence: u8) -> Expr {
    let first = tokens.next().unwrap();
    let mut left = nud(first, tokens);

    while let Some(token) = tokens.peek() {
        if token == &Token::Punctuator(Punctuator::LParen) 
        || token == &Token::Punctuator(Punctuator::RParen) {
            break;
        };

        if precedence(token) < min_precedence {
            break;
        }

        let op = tokens.next().unwrap();
        left = led(left, op, tokens);
    }

    left
}


pub fn parse_stmt(tokens: TokenIter) -> Vec<Stmt> {
    while let Some(token) = tokens.peek() {
        let stmt = match token {
            &Token::Keyword(keyword) => match keyword {
                Keyword::Let => Stmt::Let { variable: tokens.next(), expr: () }
            }
        };
    };
}