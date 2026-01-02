use crate::lex;
use super::def::*;
use crate::err::*;

impl<'a> Ast<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg: cfg }
    }

    pub fn parse(&self, input: &lex::Output) -> Result<Program, Error> {
        let mut parser = Parser::new(self, input);

        // Parse the input.
        parser.parse()?;

        let program = Program {
            name: input.name.clone(),
            body: parser.stmts,
        };

        Ok(program)
    }
}