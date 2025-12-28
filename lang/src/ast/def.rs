#[derive(Debug)]
pub struct Program {
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Var {
        name: String,
        value: Expr,
    },

    Fix {
        name: String,
        value: Expr,
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
        op: String,
        rt: Box<Expr>,
    },

    Lit {
        id: u16,
        val: String,
    },

    Ident(String),
}