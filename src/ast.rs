use crate::token::{Literal, MathOp};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal {
        literal: Literal,
    },
    Variable {
        name: String,
    },
    Eq {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ne {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Lt {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Le {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Gt {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ge {
        left: Box<Expr>,
        right: Box<Expr>,
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

// Using StmtKind because statement struct should have line number inside 
#[derive(Debug, PartialEq)]
pub enum StmtKind {
    Let {
        variable: String,
        expr: Box<Expr>,
    },
    If {
        condition: Expr,
        then_branch: Box<Vec<Stmt>>,
        else_branch: Option<Box<Vec<Stmt>>>,
    },
    Print {
        values: Vec<Expr>,
    },
    Input {
        prompt: Option<String>,
        variables: Vec<String>,
    },
    Goto {
        line: u8,
    },
    Gosub {
        line: u8,
    },
    Return,
    End,
}

#[derive(Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub line_number: u8
}