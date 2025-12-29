use crate::tok;

#[derive(Debug)]
pub struct Config {
    pub allow: [u8; 128],
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub cfg: &'a Config,
}

#[derive(Debug)]
pub struct Input<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Debug)]
pub struct Output {
    pub name: String,
    pub toks: Vec<tok::Tok>,
}