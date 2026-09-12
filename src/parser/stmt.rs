use std::array;
use std::borrow::Cow;

use crate::ast::{Expr, RawStmt, Stmt};
use crate::parser::expr::parse_expr;
use crate::parser::utils::is_expr_token;
use crate::token::{Keyword, Literal, Punctuator, Token, TokenIter};

fn parse_let_raw_stmt<'a>(iter: &mut TokenIter) -> Result<RawStmt, Cow<'a, str>> {
    let tokens: [Token; 3] = array::from_fn(
        |_| match iter.next() {
            Some(t) => t,
            None => unreachable!("unexcepted EOF in parse_let_raw_stmt")
        }
    );

    let expr: Expr = parse_expr(iter)?;

    match tokens {
        [Token::Keyword(Keyword::Let), Token::Ident(name), Token::Eq] => {
            Ok(RawStmt::Let {
                variable: name,
                expr: Box::new(expr),
            })
        },
        [Token::Keyword(Keyword::Let), Token::Ident(_), ..] => {
            Err(Cow::Borrowed("= excepted"))
        }
        [Token::Keyword(Keyword::Let), ..] => {
            Err(Cow::Borrowed("identifier excepted"))
        }
        _ => unreachable!("parse_let_raw_stmt should be used on let statement"),
    }
}

fn parse_if_raw_stmt<'a>(iter: &mut TokenIter) -> Result<RawStmt, Cow<'a, str>> {
    let if_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_if_raw_stmt while fetching if_token")
    };
    let condition = parse_expr(iter)?;

    let then_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_if_raw_stmt while fetching then_token")
    };
    let then_branch = parse_raw_stmt(iter)?;

    let else_token;
    let else_branch;

    if iter.peek() == Some(&Token::Keyword(Keyword::Else)) {
        else_token = iter.next();
        else_branch = Some(parse_raw_stmt(iter)?);
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
            Ok(RawStmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: Some(Box::new(else_branch.unwrap())),
            })
        },
        (Token::Keyword(Keyword::If), Token::Keyword(Keyword::Then), None) => {
            Ok(RawStmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: None,
            })
        },
        (Token::Keyword(Keyword::If), ..) => {
            Err(Cow::Borrowed("keyword 'THEN' excepted"))
        }
        _ => unreachable!("parse_let_raw_stmt should be used on let statement"),
    }
}

fn parse_print_raw_stmt<'a>(iter: &mut TokenIter) -> Result<RawStmt, Cow<'a, str>> {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_print_raw_stmt")
    };

    let mut values: Vec<Expr> = Vec::new();

    while let Some(token) = iter.peek() {
        if is_expr_token(token) {
            values.push(parse_expr(iter)?);
        } else if token == &Token::Punctuator(Punctuator::Comma) {
            iter.next();
        } else {
            break;
        }
    }

    match token {
        Token::Keyword(Keyword::Print) => Ok(RawStmt::Print { values }),
        _ => unreachable!("parse_print_raw_stmt should be used on print statement"),
    }
}

fn parse_input_raw_stmt(iter: &mut TokenIter) -> RawStmt {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_input_raw_stmt")
    };

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
        Token::Keyword(Keyword::Input) => RawStmt::Input { prompt, variables },
        _ => unreachable!("parse_input_raw_stmt should be used on input statement"),
    }
}

fn parse_goto_raw_stmt<'a>(iter: &mut TokenIter) -> Result<RawStmt, Cow<'a, str>> {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_input_raw_stmt while fetching goto token")
    };

    let line_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_input_raw_stmt while fetching line number")
    };

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        token => {
            let err_msg = format!("excepted line number, found {} token", token);
            return Err(Cow::Owned(err_msg))}
        };

    match token {
        Token::Keyword(Keyword::Goto) => Ok(RawStmt::Goto { line }),
        _ => unreachable!("parse_goto_raw_stmt should be used on goto statement"),
    }
}

fn parse_gosub_raw_stmt<'a>(iter: &mut TokenIter) -> Result<RawStmt, Cow<'a, str>> {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_gosub_raw_stmt while fetching goto token")
    };

    let line_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_gosub_raw_stmt while fetching line number")
    };

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        token => {
            let err_msg = format!("excepted line number, found {} token", token);
            return Err(Cow::Owned(err_msg))}
        };

    match token {
        Token::Keyword(Keyword::Gosub) => Ok(RawStmt::Gosub { line }),
        _ => unreachable!("parse_gosub_raw_stmt should be used on gosub statement"),
    }
}

fn parse_return_raw_stmt(iter: &mut TokenIter) -> RawStmt {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_return_raw_stmt")
    };
    
    match token {
        Token::Keyword(Keyword::Return) => RawStmt::Return,
        _ => unreachable!("parse_return_raw_stmt should be used on return statement"),
    }
}

fn parse_end_raw_stmt(iter: &mut TokenIter) -> RawStmt {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_end_raw_stmt")
    };
    
    match token {
        Token::Keyword(Keyword::End) => RawStmt::End,
        _ => unreachable!("parse_end_raw_stmt should be used on end statement"),
    }
}

fn parse_raw_stmt<'a>(iter: &mut TokenIter) -> Result<RawStmt, Cow<'a, str>> {
    match iter.peek().unwrap() {
        Token::Keyword(keyword) => {
            let raw_stmt = match keyword  {
                Keyword::Let => parse_let_raw_stmt(iter)?,
                Keyword::If => parse_if_raw_stmt(iter)?,
                Keyword::Print => parse_print_raw_stmt(iter)?,
                Keyword::Input => parse_input_raw_stmt(iter),
                Keyword::Goto => parse_goto_raw_stmt(iter)?,
                Keyword::Gosub => parse_gosub_raw_stmt(iter)?,
                Keyword::Return => parse_return_raw_stmt(iter),
                Keyword::End => parse_end_raw_stmt(iter),
                Keyword::Else | Keyword::Then => {
                    let err_msg = format!("keyword '{:?}' should not be used standalone", keyword);
                    return Err(Cow::Owned(err_msg))
                }
            };
            Ok(raw_stmt)
        },
        _ => unreachable!("parse_raw_stmt should be used only on one statement")
    }
}

/// It is internal implementation and it should get iterator instead of vector.
fn parse_stmts<'a>(iter: &mut TokenIter) -> Result<Vec<Stmt>, Cow<'a, str>> {
    let mut ast: Vec<Stmt> = Vec::new();
    let mut line_number = 0;

    while let Some(token) = iter.peek() {
        match token {
            Token::Keyword(_) => {
                let raw_stmt = parse_raw_stmt(iter)?;
                let stmt = Stmt { data: raw_stmt, line_number };
                ast.push(stmt);
            },
            &Token::Literal(Literal::Num(num)) => {
                line_number = num as u8;
                iter.next();
            }
            token => {
                let err_msg = format!("unexcepted token '{}'. excepted keyword or line number", token);
                return Err(Cow::Owned(err_msg))
            },
        }
    };

    Ok(ast)
}

pub fn parse<'a>(tokens: Vec<Token>) -> Result<Vec<Stmt>, Cow<'a, str>> {
    let mut iter: TokenIter = tokens.into_iter().peekable();
    
    parse_stmts(&mut iter)
}