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

    Ident(String),
}