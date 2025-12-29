use super::def::*;
use crate::lex;
use crate::tok;

impl<'a> Parser<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self {
            ast: ast,
            idx: 0,
            stmts: vec![],
        }
    }

    fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn parse(&mut self, _input: &lex::Output) -> Option<String> {
        self.push(Stmt::Var {
            name: "a".to_string(),
            expr: Expr::Lit {
                id: tok::INT,
                val: "1".to_string(),
            },
        });

        None
    }
}