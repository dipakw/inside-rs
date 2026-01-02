use crate::lex;

pub struct Config {
    pub allow: [u8; 128],
}

pub struct Ast<'a> {
    pub cfg: &'a Config,
}

pub struct Parser<'a> {
    pub ast: &'a Ast<'a>,
    pub idx: usize,
    pub stmts: Vec<Stmt>,
    pub input: &'a lex::Output,
    pub count: usize,
}

#[derive(Debug)]
pub struct Program {
    pub name: String,
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn push(&mut self, stmt: Stmt) {
        self.body.push(stmt);
    }
}

#[derive(Debug)]
pub enum Stmt {
    Var {
        name: String,
        expr: Expr,
    },

    Fix {
        name: String,
        expr: Expr,
    },

    Set {
        name: String,
        expr: Expr,
    },

    If {
        cond: Expr,
        pass: Vec<Stmt>,
        fail: Option<Vec<Stmt>>,
    },

    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Bin {
        lf: Box<Expr>,
        op: u16,
        rt: Box<Expr>,
    },

    Lit {
        id: u16,
        val: String,
    },
    
    Call {
        name: String,
        args: Vec<Expr>,
    },

    Ident(String),
}