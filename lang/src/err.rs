#[derive(Debug)]
pub struct Error {
    pub name: String,
    pub text: String,
    pub line: u32,
    pub colm: u32,
}