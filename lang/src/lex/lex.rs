use crate::lex::Config;

use super::def::{Lexer, Input};
use crate::tok;

static IGNORE: [u16; 2] = [
    tok::SPACE,
    tok::TAB,
];

fn push(tokens: &mut Vec<tok::Tok>, buffer: &mut Vec<char>, ln: u32, col: u32, val: String, size: u32, id: u16) {
    if IGNORE.contains(&id) {
        return;
    }

    tokens.push(tok::Tok {
        id: id,
        val: val,
        ln: ln,
        col: col - size,
    });

    *buffer = vec![];
}

fn flush(tokens: &mut Vec<tok::Tok>, buffer: &mut Vec<char>, ln: u32, col: u32, str_start: bool) -> Result<(), String> {
    let val: String = buffer.iter().collect();
    let size = val.len() as u32;
    
    if val.is_empty() {
        return Ok(());
    }

    let tok: (u16, bool) = if str_start {
        (tok::STR, false)
    } else {
        tok::id(&val)
    };

    if tok.0 == 0 {
        return Err(format!("invalid token: {}", val));
    }

    push(tokens, buffer, ln, col, val, size, tok.0);

    Ok(())
}

impl<'a> Lexer<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg: cfg }
    }

    pub fn tokenize(&self, input: &Input) -> Result<Vec<tok::Tok>, String> {
        let mut tokens: Vec<tok::Tok> = vec![];
        let mut ln: u32 = 1;
        let mut col: u32 = 0;
        let mut str_start: bool = false;
        let mut str_end_if: Option<char> = None;
        let mut buffer: Vec<char> = vec![];
        let str_sps: Vec<u16> = vec![tok::DQUOTE, tok::TICK];

        for char in input.code.chars() {
            col += 1;

            let char_str = char.to_string();
            let (id, sep) = tok::id(&char_str);

            if str_start && str_end_if.is_some() && char != str_end_if.unwrap() {
                buffer.push(char);
                if id == tok::EOL {
                    ln += 1;
                }
                continue;
            }

            if str_sps.contains(&id) {
                if str_end_if.is_some() && char == str_end_if.unwrap() {
                    flush(&mut tokens, &mut buffer, ln, col, str_start)?;
                    str_start = false;
                    str_end_if = None;
                    continue;
                }

                if !str_start {
                    str_start = true;
                    str_end_if = Some(char);
                    continue;
                }
            }

            if sep {
                // Flush buffer
                flush(&mut tokens, &mut buffer, ln, col, str_start)?;

                // Push the separator
                push(&mut tokens, &mut buffer, ln, col, "".to_string(), 0, id);

                // If line break
                if id == tok::EOL {
                    ln += 1;
                    col = 0;
                }

                continue;
            }

            buffer.push(char);
        }

        col += 1;

        // If string was started and reached the end of the file,
        // return error
        if str_start {
            return Err("unexpected end of file".to_string());
        }

        // Flush buffer
        flush(&mut tokens, &mut buffer, ln, col, str_start)?;

        // Push EOF
        push(&mut tokens, &mut buffer, ln, col, "".to_string(), 0, tok::EOF);

        Ok(tokens)
    }
}