use super::def::*;
use crate::err::*;
use crate::lex;
use crate::tok;

// Mutiline spaces.
static MLS: [u16; 3] = [tok::SPACE, tok::TAB, tok::EOL];

// Inline spaces.
static ILS: [u16; 2] = [tok::SPACE, tok::TAB];

// Flags.
static NO: u16 = 0;
static IL: u16 = 1;
static ML: u16 = 2;

// Binary operators.
static BIN_OPS: &[u16] = &[
    tok::PLUS,
    tok::MINUS,
    tok::MUL,
    tok::DIV,
];

impl<'a> Parser<'a> {
    pub fn new(ast: &'a Ast, input: &'a lex::Output) -> Self {
        Self {
            ast: ast,
            idx: 0,
            stmts: vec![],
            input: input,
            count: input.toks.len(),
        }
    }

    pub fn parse(&mut self) -> Option<Error> {
        while self.idx < self.count {
            // Skip the spaces, including line breaks.
            self.burn(&MLS);

            // Get the current token.
            let tok = &self.input.toks[self.idx];

            match tok.id {
                tok::EOF => break,

                tok::FIX | tok::VAR => match self.parse_var_fix() {
                    Some(error) => return Some(error),
                    None => {}
                },

                _ => {
                    return Some(self.err_unexpected_tok(0));
                }
            }
        }

        None
    }

    fn err_unexpected_tok(&self, off: usize) -> Error {
        let tok = &self.input.toks[self.idx + off];
        let mut text = format!("unexpected \"{}\" ({})", tok.val, tok.id);

        if tok.id == tok::EOF {
            text = "unexpected end of file".to_string();
        }

        Error {
            name: self.input.name.clone(),
            text: text,
            line: tok.ln,
            colm: tok.col,
        }
    }

    fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    fn burn(&mut self, ids: &[u16]) -> &Self {
        while ids.contains(&self.input.toks[self.idx].id) {
            self.idx += 1;
        }

        self
    }

    // Dry burn.
    fn burnd(&self, off: usize, ids: &[u16]) -> usize {
        let mut count = 0;

        while ids.contains(&self.input.toks[off + count].id) {
            count += 1;
        }

        count
    }

    fn peek(&self, off: usize, seq: &[&[u16]]) -> bool {
        let mut idx = self.idx + off;

        for i in 0..seq.len() {
            let tok = &self.input.toks[idx + i];
            let ids = &seq[i][..seq[i].len() - 1];
            let spc = seq[i][seq[i].len() - 1];

            idx += 1;

            if !ids.contains(&tok.id) {
                return false;
            }

            if spc == IL {
                idx += self.burnd(idx, &ILS)
            } else if spc == ML {
                idx += self.burnd(idx, &MLS)
            }
        }

        true
    }

    // Peek single.
    fn peeks(&self, off: usize, ids: &[u16]) -> bool {
        ids.contains(&self.input.toks[self.idx + off].id)
    }

    // Peek single sequence.
    fn peeksq(&self, off: usize, seq: &[u16]) -> bool {
        for i in 0..seq.len() {
            if !self.peeks(off + i, &[seq[i]]) {
                return false;
            }
        }

        true
    }

    fn grab(&mut self, seq: &[&[u16]]) -> Result<Vec<&tok::Tok>, Error> {
        let mut toks: Vec<&tok::Tok> = vec![];

        for i in 0..seq.len() {
            let tok = &self.input.toks[self.idx];
            let ids = &seq[i][..seq[i].len() - 1];
            let spc = seq[i][seq[i].len() - 1];

            if !ids.contains(&tok.id) {
                return Err(self.err_unexpected_tok(i));
            }

            self.idx += 1;

            if spc == IL {
                self.burn(&ILS);
            } else if spc == ML {
                self.burn(&MLS);
            }

            toks.push(tok);
        }

        Ok(toks)
    }

    fn parse_var_fix(&mut self) -> Option<Error> {
        let toks = match self.grab(&[
            &[tok::VAR, tok::FIX, IL],
            &[tok::IDENT, IL],
            &[tok::EQUAL, IL],
        ]) {
            Ok(toks) => toks,
            Err(error) => return Some(error),
        };

        let (name, tok_id) = (toks[1].val.clone(), toks[0].id);

        match self.parse_expr() {
            Ok(expr) => match tok_id {
                tok::VAR => {
                    self.push(Stmt::Var { name, expr });
                }

                tok::FIX => {
                    self.push(Stmt::Fix { name, expr });
                }

                _ => {}
            },

            Err(error) => return Some(error),
        };

        None
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_expr_atom()?;

        // Parse binary operators left-associatively.
        while self.peeks(0, &BIN_OPS) {
            let grab = &[BIN_OPS, &[ML]].concat();

            let tok = match self.grab(&[grab]) {
                Ok(toks) => toks[0],
                Err(error) => return Err(error),
            };

            let tokid = tok.id;
            let right = self.parse_expr_atom()?;

            left = Expr::Bin {
                lf: Box::new(left),
                op: tokid,
                rt: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_expr_atom(&mut self) -> Result<Expr, Error> {
        let mut expr: Option<Expr> = None;

        // Function call
        // <ident>(
        if self.peeksq(0, &[tok::IDENT, tok::LPAREN]) {
            match self.parse_call_expr() {
                Ok(call) => expr = Some(call),
                Err(error) => return Err(error),
            };
        }

        // Parenthesized expression
        // (<expr>)
        if self.peeks(0, &[tok::LPAREN]) {
            let _ = self.grab(&[&[tok::LPAREN, ML]]);
            let inner_expr = self.parse_expr()?;

            let _ = match self.grab(&[&[tok::RPAREN, ML]]) {
                Ok(_) => {},
                Err(error) => return Err(error),
            };

            expr = Some(inner_expr);
        }

        // <int>.<int>
        if self.peeksq(0, &[tok::INT, tok::DOT, tok::INT]) {
            let toks = match self.grab(&[&[tok::INT, IL], &[tok::DOT, IL], &[tok::INT, ML]]) {
                Ok(toks) => toks,
                Err(error) => return Err(error),
            };

            expr = Some(Expr::Lit {
                id: tok::FLOAT,
                val: format!("{}.{}", toks[0].val, toks[2].val),
            });
        }

        // <bool> | <str> | <int>
        if self.peeks(0, &[tok::BOOL, tok::STR, tok::INT]) {
            let tok = match self.grab(&[&[tok::BOOL, tok::STR, tok::INT, ML]]) {
                Ok(toks) => toks[0],
                Err(error) => return Err(error),
            };

            expr = Some(Expr::Lit {
                id: tok.id,
                val: tok.val.clone(),
            });
        }

        // <ident>
        if self.peeks(0, &[tok::IDENT]) {
            match self.grab(&[&[tok::IDENT, ML]]) {
                Ok(toks) => expr = Some(Expr::Ident(toks[0].val.clone())),
                Err(error) => return Err(error),
            };
        }

        if expr.is_none() {
            return Err(self.err_unexpected_tok(0));
        }

        Ok(expr.unwrap())
    }

    fn parse_call_expr(&mut self) -> Result<Expr, Error> {
        // Parse: <ident>(
        let toks = match self.grab(&[&[tok::IDENT, IL], &[tok::LPAREN, ML]]) {
            Ok(toks) => toks,
            Err(error) => return Err(error),
        };

        let name = toks[0].val.clone();

        let args = match self.parse_val_args(tok::COMMA, tok::RPAREN) {
            Ok(args) => args,
            Err(error) => return Err(error),
        };

        Ok(Expr::Call {
            name: name,
            args: args,
        })
    }

    fn parse_val_args(&mut self, sep: u16, end: u16) -> Result<Vec<Expr>, Error> {
        let mut args: Vec<Expr> = vec![];

        while !self.peeks(0, &[end]) {
            let expr = match self.parse_expr() {
                Ok(expr) => expr,
                Err(error) => return Err(error),
            };

            let mut sep_found = false;

            if self.peeks(0, &[sep]) {
                let _ = self.grab(&[&[sep, ML]]);
                sep_found = true;
            }

            if sep_found && self.peeks(0, &[end]) {
                return Err(self.err_unexpected_tok(0));
            }

            args.push(expr);
        }

        if !self.peeks(0, &[end]) {
            return Err(self.err_unexpected_tok(0));
        }

        let _ = self.grab(&[&[end, ML]]);

        Ok(args)
    }

    // parse_type_args
    // parse_named_type_args
}
