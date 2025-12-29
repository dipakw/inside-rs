use crate::lex;
use super::def::*;

impl<'a> Ast<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg: cfg }
    }

    pub fn parse(&self, input: &lex::Output) -> Result<Program, String> {
        let mut parser = Parser::new(self);

        match parser.parse(input) {
            Some(error) => return Err(error),
            None => {},
        };

        let program = Program {
            name: input.name.clone(),
            body: parser.stmts,
        };

        Ok(program)
    }
}