pub struct Config {
    pub allow: [u8; 128],
}

pub struct Lexer<'a> {
    pub cfg: &'a Config,
}

pub struct Input<'a> {
    pub name: &'a str,
    pub code: &'a str,
}