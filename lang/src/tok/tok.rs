use std::collections::HashMap;
use std::sync::LazyLock;

pub struct Tok {
    pub id: u16,
    pub val: String,
    pub ln: u32,
    pub col: u32,
}

// Constants
pub const SEP: u16 = 1; // Separator yes

// Dictionary tokens
pub const EOF: u16 = 100;
pub const IDENT: u16 = 101;
pub const INT: u16 = 102;
pub const BOOL: u16 = 103;
pub const STR: u16 = 104;

// Keywords
pub const FIX: u16 = 200;
pub const VAR: u16 = 201;

// Signs
pub const EQUAL: u16 = 300;
pub const PLUS: u16 = 301;
pub const MINUS: u16 = 302;
pub const MUL: u16 = 303;
pub const DIV: u16 = 304;
pub const LESS: u16 = 305;
pub const GT: u16 = 306;
pub const NOT: u16 = 307;
pub const COL: u16 = 308;
pub const SEMCOL: u16 = 309;
pub const DOT: u16 = 310;
pub const COMMA: u16 = 311;
pub const LPAREN: u16 = 312;
pub const RPAREN: u16 = 313;
pub const LBRACE: u16 = 314;
pub const RBRACE: u16 = 315;
pub const LSQB: u16 = 316;
pub const RSQB: u16 = 317;
pub const PIPE: u16 = 318;
pub const AMP: u16 = 319;
pub const CARET: u16 = 320;
pub const PERCENT: u16 = 321;
pub const HASH: u16 = 322;
pub const DQUOTE: u16 = 323;
pub const SQUOTE: u16 = 324;
pub const TICK: u16 = 325;
pub const BSLASH: u16 = 326;
pub const TILDE: u16 = 327;

// Whitespace
pub const SPACE: u16 = 400;
pub const TAB: u16 = 401;
pub const EOL: u16 = 402;

static MAPPING: LazyLock<HashMap<&str, Vec<u16>>> = LazyLock::new(|| HashMap::from([
    // Dictionary
    ("EOF", vec![EOF, SEP]),
    ("ident", vec![IDENT]),
    ("int", vec![INT]),
    ("bool", vec![BOOL]),
    ("str", vec![STR]),
    
    // Keywords
    ("fix", vec![FIX]),
    ("var", vec![VAR]),
    
    // Signs
    ("=", vec![EQUAL, SEP]),
    ("+", vec![PLUS, SEP]),
    ("-", vec![MINUS, SEP]),
    ("*", vec![MUL, SEP]),
    ("/", vec![DIV, SEP]),
    ("<", vec![LESS, SEP]),
    (">", vec![GT, SEP]),
    ("!", vec![NOT, SEP]),
    (":", vec![COL, SEP]),
    (";", vec![SEMCOL, SEP]),
    (".", vec![DOT, SEP]),
    (",", vec![COMMA, SEP]),
    ("(", vec![LPAREN, SEP]),
    (")", vec![RPAREN, SEP]),
    ("{", vec![LBRACE, SEP]),
    ("}", vec![RBRACE, SEP]),
    ("[", vec![LSQB, SEP]),
    ("]", vec![RSQB, SEP]),
    ("|", vec![PIPE, SEP]),
    ("&", vec![AMP, SEP]),
    ("^", vec![CARET, SEP]),
    ("%", vec![PERCENT, SEP]),
    ("#", vec![HASH, SEP]),
    ("\"", vec![DQUOTE, SEP]),
    ("'", vec![SQUOTE, SEP]),
    ("`", vec![TICK, SEP]),
    ("\\", vec![BSLASH, SEP]),
    ("~", vec![TILDE, SEP]),
    
    // Whitespace
    (" ", vec![SPACE, SEP]),
    ("\t", vec![TAB, SEP]),
    ("\n", vec![EOL, SEP]),
]));

pub fn id(name: &str) -> (u16, bool) {
    let id = MAPPING.get(name); 

    if id.is_none() {
        if is_ident(name) {
            return (IDENT, false);
        }
        
        if is_int(name) {
            return (INT, false);
        }

        return (0, false);
    }

    let id = id.unwrap();

    if id.len() > 1 {
        return (id[0], id[1] == SEP);
    }

    return (id[0], false);
}

pub fn is_ident(tok: &str) -> bool {
	if tok.is_empty() {
		return false;
	}

	let mut chars = tok.chars();
	
	// First character: letter or underscore
	let first = chars.next().unwrap();
	if !(first == '_' || first.is_ascii_alphabetic()) {
		return false;
	}

	// Remaining characters: letters, digits, or underscore
	for c in chars {
		if !(c == '_' || c.is_ascii_alphanumeric()) {
			return false;
		}
	}

	true
}

pub fn is_int(tok: &str) -> bool {
	!tok.is_empty() && tok.chars().all(|c| c.is_ascii_digit())
}
