use std::array;
use std::borrow::Cow;

use crate::ast::{Expr, StmtKind, Stmt};
use crate::parser::expr::parse_expr;
use crate::parser::utils::is_expr_token;
use crate::token::{Keyword, Literal, Punctuator, Token, TokenIter};

fn parse_let_stmt_kind<'a>(iter: &mut TokenIter) -> Result<StmtKind, Cow<'a, str>> {
    let tokens: [Token; 3] = array::from_fn(
        |_| match iter.next() {
            Some(t) => t,
            None => unreachable!("unexcepted EOF in parse_let_stmt_kind")
        }
    );

    let expr: Expr = parse_expr(iter)?;

    match tokens {
        [Token::Keyword(Keyword::Let), Token::Ident(name), Token::Eq] => {
            Ok(StmtKind::Let {
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
        _ => unreachable!("parse_let_stmt_kind should be used on let statement"),
    }
}

fn parse_if_stmt_kind<'a>(iter: &mut TokenIter) -> Result<StmtKind, Cow<'a, str>> {
    let if_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_if_stmt_kind while fetching if_token")
    };
    let condition = parse_expr(iter)?;

    let then_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_if_stmt_kind while fetching then_token")
    };
    let then_branch = parse_stmts(iter)?;

    let else_token;
    let else_branch;

    if iter.peek() == Some(&Token::Keyword(Keyword::Else)) {
        else_token = iter.next();
        else_branch = Some(parse_stmts(iter)?);
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
            Ok(StmtKind::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: Some(Box::new(else_branch.unwrap())),
            })
        },
        (Token::Keyword(Keyword::If), Token::Keyword(Keyword::Then), None) => {
            Ok(StmtKind::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: None,
            })
        },
        (Token::Keyword(Keyword::If), ..) => {
            Err(Cow::Borrowed("keyword 'THEN' excepted"))
        }
        _ => unreachable!("parse_let_stmt_kind should be used on let statement"),
    }
}

fn parse_print_stmt_kind<'a>(iter: &mut TokenIter) -> Result<StmtKind, Cow<'a, str>> {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_print_stmt_kind")
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
        Token::Keyword(Keyword::Print) => Ok(StmtKind::Print { values }),
        _ => unreachable!("parse_print_stmt_kind should be used on print statement"),
    }
}

fn parse_input_stmt_kind(iter: &mut TokenIter) -> StmtKind {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_input_stmt_kind")
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
        Token::Keyword(Keyword::Input) => StmtKind::Input { prompt, variables },
        _ => unreachable!("parse_input_stmt_kind should be used on input statement"),
    }
}

fn parse_goto_stmt_kind<'a>(iter: &mut TokenIter) -> Result<StmtKind, Cow<'a, str>> {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_input_stmt_kind while fetching goto token")
    };

    let line_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_input_stmt_kind while fetching line number")
    };

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        token => {
            let err_msg = format!("excepted line number, found {} token", token);
            return Err(Cow::Owned(err_msg))}
        };

    match token {
        Token::Keyword(Keyword::Goto) => Ok(StmtKind::Goto { line }),
        _ => unreachable!("parse_goto_stmt_kind should be used on goto statement"),
    }
}

fn parse_gosub_stmt_kind<'a>(iter: &mut TokenIter) -> Result<StmtKind, Cow<'a, str>> {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_gosub_stmt_kind while fetching goto token")
    };

    let line_token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_gosub_stmt_kind while fetching line number")
    };

    let line = match line_token {
        Token::Literal(Literal::Num(value)) => value as u8,
        token => {
            let err_msg = format!("excepted line number, found {} token", token);
            return Err(Cow::Owned(err_msg))}
        };

    match token {
        Token::Keyword(Keyword::Gosub) => Ok(StmtKind::Gosub { line }),
        _ => unreachable!("parse_gosub_stmt_kind should be used on gosub statement"),
    }
}

fn parse_return_stmt_kind(iter: &mut TokenIter) -> StmtKind {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_return_stmt_kind")
    };
    
    match token {
        Token::Keyword(Keyword::Return) => StmtKind::Return,
        _ => unreachable!("parse_return_stmt_kind should be used on return statement"),
    }
}

fn parse_end_stmt_kind(iter: &mut TokenIter) -> StmtKind {
    let token = match iter.next() {
        Some(t) => t,
        None => unreachable!("unexcepted EOF in parse_end_stmt_kind")
    };
    
    match token {
        Token::Keyword(Keyword::End) => StmtKind::End,
        _ => unreachable!("parse_end_stmt_kind should be used on end statement"),
    }
}

/// It is internal implementation and it should get iterator instead of vector.
fn parse_stmts<'a>(iter: &mut TokenIter) -> Result<Vec<Stmt>, Cow<'a, str>> {
    let mut ast: Vec<Stmt> = Vec::new();
    let mut line_number = 0;

    while let Some(token) = iter.peek() {
        match token {
            Token::Keyword(keyword) => {
                let stmt_kind = match keyword  {
                    Keyword::Let => parse_let_stmt_kind(iter)?,
                    Keyword::If => parse_if_stmt_kind(iter)?,
                    Keyword::Print => parse_print_stmt_kind(iter)?,
                    Keyword::Input => parse_input_stmt_kind(iter),
                    Keyword::Goto => parse_goto_stmt_kind(iter)?,
                    Keyword::Gosub => parse_gosub_stmt_kind(iter)?,
                    Keyword::Return => parse_return_stmt_kind(iter),
                    Keyword::End => parse_end_stmt_kind(iter),
                    Keyword::Else | Keyword::Then => {
                        let err_msg = format!("keyword '{:?}' should not be used standalone", keyword);
                        return Err(Cow::Owned(err_msg));
                    }
                };
                let stmt = Stmt { kind: stmt_kind, line_number };
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