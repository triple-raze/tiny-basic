use std::array;

use crate::ast::{Expr, Stmt};
use crate::parser::expr::parse_expr;
use crate::parser::utils::{TokenIter, is_expr_token};
use crate::token::{Keyword, Literal, Punctuator, Token};

fn parse_let_stmt(iter: &mut TokenIter) -> Result<Stmt, &'static str> {
    let tokens: [Token; 3] = array::from_fn(|_| iter.next().unwrap());

    let expr: Expr = parse_expr(iter)?;

    match tokens {
        [Token::Keyword(Keyword::Let), Token::Ident(name), Token::Eq] => {
            Ok(Stmt::Let {
                variable: name,
                expr: Box::new(expr),
            })
        },
        [Token::Keyword(Keyword::Let), Token::Ident(_), ..] => {
            Err("= excepted")
        }
        [Token::Keyword(Keyword::Let), ..] => {
            Err("identifier excepted")
        }
        _ => unreachable!("parse_let_stmt should be used on let statement"),
    }
}

fn parse_if_stmt(iter: &mut TokenIter) -> Result<Stmt, &'static str> {
    let if_token = iter.next().unwrap();
    let condition = parse_expr(iter);

    let then_token = iter.next().unwrap();

    let then_branch = parse_internal(iter)?;

    let else_token;
    let else_branch;

    if iter.peek() == Some(&Token::Keyword(Keyword::Else)) {
        else_token = iter.next();
        else_branch = Some(parse_internal(iter)?);
    } else {
        else_token = None;
        else_branch = None;
    }

    match (if_token, then_token, else_token) {
        (
            Token::Keyword(Keyword::If),
            Token::Keyword(Keyword::Then),
            Some(Token::Keyword(Keyword::Else)),
        ) => {
            Ok(Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: Some(Box::new(else_branch.unwrap())),
            })
        },
        (Token::Keyword(Keyword::If), Token::Keyword(Keyword::Then), None) => {
            Ok(Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: None,
            })
        },
        (Token::Keyword(Keyword::If), ..) => {
            Err("keyword 'THEN' excepted")
        }
        _ => unreachable!("parse_let_stmt should be used on let statement"),
    }
}

fn parse_print_stmt(iter: &mut TokenIter) -> Stmt {
    let token = iter.next().unwrap();

    let mut values: Vec<Expr> = Vec::new();

    while let Some(token) = iter.peek() {
        if is_expr_token(token) {
            values.push(parse_expr(iter));
        } else if token == &Token::Punctuator(Punctuator::Comma) {
            iter.next();
        } else {
            break;
        }
    }

    match token {
        Token::Keyword(Keyword::Print) => Stmt::Print { values },
        _ => unreachable!("parse_print_stmt should be used on print statement"),
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
        _ => unreachable!("parse_input_stmt should be used on input statement"),
    }
}

fn parse_goto_stmt(iter: &mut TokenIter) -> Result<Stmt, &'static str> {
    let token = iter.next().unwrap();
    let line_token = iter.next().unwrap();

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        _ => return Err("line number excepted after goto statement"),
    };

    match token {
        Token::Keyword(Keyword::Goto) => Ok(Stmt::Goto { line }),
        _ => unreachable!("parse_goto_stmt should be used on goto statement"),
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

/// It is internal implementation and it should get iterator instead of vector
fn parse_internal(iter: &mut TokenIter) -> Result<Vec<Stmt>, &'static str> {
    let mut ast: Vec<Stmt> = Vec::new();

    while let Some(token) = iter.peek() {
        match token {
            Token::Keyword(keyword) => {
                let stmt = match keyword  {
                    Keyword::Let => parse_let_stmt(iter)?,
                    Keyword::If => parse_if_stmt(iter)?,
                    Keyword::Print => parse_print_stmt(iter),
                    Keyword::Input => parse_input_stmt(iter),
                    Keyword::Goto => parse_goto_stmt(iter)?,
                    Keyword::Gosub => parse_gosub_stmt(iter),
                    Keyword::Return => parse_return_stmt(iter),
                    Keyword::End => parse_end_stmt(iter),
                    Keyword::Else | Keyword::Then => panic!("Keyword {:?} should be only used afte blah blah blah", keyword)
                };
                ast.push(stmt);
            },
            token => panic!("parse_stmt {:?}", token),
        }
    };

    Ok(ast)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, &'static str> {
    let mut iter: TokenIter = tokens.into_iter().peekable();
    
    parse_internal(&mut iter)
}