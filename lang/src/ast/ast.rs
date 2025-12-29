use crate::tok;
use crate::lex;
use super::def::*;

pub struct Config {
    pub allow: [u8; 128],
}

pub struct Ast<'a> {
    pub cfg: &'a Config,
}

impl<'a> Ast<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg: cfg }
    }

    pub fn parse(&self, input: &lex::Output) -> Result<Program, String> {
        let mut program = Program {
            name: input.name.clone(),
            body: vec![],
        };

        program.body.push(Stmt::Var {
            name: "a".to_string(),
            expr: Expr::Lit {
                id: tok::INT,
                val: "1".to_string(),
            },
        });

        Ok(program)
    }
}