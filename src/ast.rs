use crate::token::{Literal, MathOp};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal {
        literal: Literal,
    },
    Variable {
        name: String,
    },
    BinOp {
        op: MathOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: MathOp,
        expr: Box<Expr>,
    },
}
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let {
        variable: String,
        expr: Box<Expr>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Input {
        prompt: Option<String>,
        variables: Vec<String>,
    },
    Print {
        values: Vec<Expr>,
    },
    Goto {
        line: u16,
    },
    Gosub {
        line: u16,
    },
    Return,
    End,
}
