use std::array;

use crate::ast::{Expr, Stmt};
use crate::parser::expr::parse_expr;
use crate::parser::utils::{TokenIter, is_expr_token};
use crate::token::{Keyword, Literal, Token, Punctuator};

fn take_expr_tokens(iter: &mut TokenIter) -> Vec<Token> {
    let mut expr_tokens = Vec::new();

    while let Some(token) = iter.peek() {
        if is_expr_token(token) {
            expr_tokens.push(iter.next().unwrap());
        } else {
            break;
        }
    };

    expr_tokens
}

fn parse_let_stmt(iter: &mut TokenIter) -> Stmt {
    let tokens: [Token; 3] = array::from_fn(|_| iter.next().unwrap());

    let expr: Expr = parse_expr(take_expr_tokens(iter));

    match tokens {
        [Token::Keyword(Keyword::Let), Token::Ident(name), Token::Eq] => Stmt::Let {
            variable: name,
            expr: Box::new(expr),
        },
        [Token::Keyword(Keyword::Let), Token::Ident(_), ..] => {
            eprintln!("Syntax Error: = excepted");
            std::process::exit(1)
        }
        [Token::Keyword(Keyword::Let), ..] => {
            eprintln!("Syntax Error: identifier excepted");
            std::process::exit(1)
        }
        _ => panic!("parse_let_stmt should be used on let statement"),
    }
}

fn parse_if_stmt(iter: &mut TokenIter) -> Stmt {
    let first = iter.next().unwrap();
    let condition = parse_expr(take_expr_tokens(iter));

    let second = iter.next().unwrap();
    
    let then_branch = parse_stmt(iter);

    let third;
    let else_branch;

    if iter.peek() == Some(&Token::Keyword(Keyword::Else)) {
        third = iter.next();
        else_branch = Some(parse_stmt(iter));
    } else {
        third = None;
        else_branch = None;
    }

    match (first, second, third) {
        (
            Token::Keyword(Keyword::If),
            Token::Keyword(Keyword::Then),
            Some(Token::Keyword(Keyword::Else)),
            ..,
        ) => Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Some(Box::new(else_branch.unwrap())),
        },

        (Token::Keyword(Keyword::If), Token::Keyword(Keyword::Then), None, ..) => Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: None,
        },
        (Token::Keyword(Keyword::If), ..) => {
            todo!()
        }
        _ => panic!("parse_let_stmt should be used on let statement"),
    }
}

fn parse_print_stmt(iter: &mut TokenIter) -> Stmt {
    let token = iter.next().unwrap();

    let mut values: Vec<Expr> = Vec::new();

    while let Some(token) = iter.peek() {
        if is_expr_token(token) {
            values.push(parse_expr(take_expr_tokens(iter)));
        } else if token == &Token::Punctuator(Punctuator::Comma) {
            continue;
        } else {
            break;
        }
    }

    match token {
        Token::Keyword(Keyword::Print) => Stmt::Print { values },
        _ => panic!(),
    }
}

fn parse_input_stmt(iter: &mut TokenIter) -> Stmt {
    let token = iter.next().unwrap();

    let prompt = match iter.next() {
        Some(Token::Literal(Literal::Str(name))) => Some(name),
        _ => None,
    };

    let variables = iter
        .by_ref()
        .map_while(|item| match item {
            Token::Literal(Literal::Str(name)) => Some(name),
            _ => None,
        })
        .collect();

    match token {
        Token::Keyword(Keyword::Input) => Stmt::Input { prompt, variables },
        _ => panic!(),
    }
}

fn parse_goto_stmt(iter: &mut TokenIter) -> Stmt {
    let token = iter.next().unwrap();
    let line_token = iter.next().unwrap();

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        _ => panic!(),
    };

    match token {
        Token::Keyword(Keyword::Goto) => Stmt::Goto { line },
        _ => panic!(),
    }
}

fn parse_gosub_stmt(iter: &mut TokenIter) -> Stmt {
    let token = iter.next().unwrap();
    let line_token = iter.next().unwrap();

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        _ => panic!(),
    };

    match token {
        Token::Keyword(Keyword::Gosub) => Stmt::Gosub { line },
        _ => panic!(),
    }
}

fn parse_return_stmt(iter: &mut TokenIter) -> Stmt {
    match iter.next().unwrap() {
        Token::Keyword(Keyword::Return) => Stmt::Return,
        _ => panic!(),
    }
}

fn parse_end_stmt(iter: &mut TokenIter) -> Stmt {
    match iter.next().unwrap() {
        Token::Keyword(Keyword::End) => Stmt::End,
        _ => panic!(),
    }
}

pub fn parse_stmt(iter: &mut TokenIter) -> Stmt {
    match iter.peek().unwrap() {
        Token::Keyword(Keyword::Let) => parse_let_stmt(iter),
        Token::Keyword(Keyword::If) => parse_if_stmt(iter),
        Token::Keyword(Keyword::Print) => parse_print_stmt(iter),
        Token::Keyword(Keyword::Input) => parse_input_stmt(iter),
        Token::Keyword(Keyword::Goto) => parse_goto_stmt(iter),
        Token::Keyword(Keyword::Gosub) => parse_gosub_stmt(iter),
        Token::Keyword(Keyword::Return) => parse_return_stmt(iter),
        Token::Keyword(Keyword::End) => parse_end_stmt(iter),
        a => panic!("parse_stmt {:?}", a),
    }
}
