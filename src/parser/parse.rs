use crate::ast::Stmt;
use crate::parser::stmt::parse_stmt;
use crate::parser::utils::TokenIter;
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut iter: TokenIter = tokens.into_iter().peekable();

    let mut ast: Vec<Stmt> = Vec::new();

    while let Some(token) = iter.peek() {
        let node = match token {
            &Token::Keyword(_) => parse_stmt(&mut iter),
            _ => panic!("unknown shit at parse fn: {:?}", token),
        };

        ast.push(node);
    }

    ast
}
