use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenGroupLexer {
    Keywords(KeywordsGroup),
    Delimiters(DelimitersGroup),
    Variables(String),
    Identifier(String),
    Illegal((char, usize, usize)),
    Eof
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenGroup {
    Keywords(KeywordsGroup),
    Delimiters(DelimitersGroup),
    Variables(u64),
    Identifier(u64),
    Eof
}

pub struct TokenLexer {
    pub token: TokenGroupLexer,
    pub line: usize,
    pub column: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token: TokenGroup,
    pub line: usize,
    pub column: usize
}

impl Token {
    pub fn eof() -> Self {
        Self {
            token: TokenGroup::Eof,
            line: 0,
            column: 0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordsGroup {
    Integer,
    Real,
    Boolean,
    True,
    False,
    Var,
    Let,
    If,
    Then,
    Else,
    EndElse,
    For,
    Do,
    While,
    Loop,
    Input,
    Output
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimitersGroup {
    Semicolon,         // ";"
    Colon,             // ":"
    Comma,             // ","
    NotEqual,          // "!="
    Equal,             // "="
    Identical,         // "=="
    Less,              // "<"
    Greater,           // ">"
    LessEqual,         // "<="
    GreaterEqual,      // ">="
    LeftParenthesis,   // "("
    RightParenthesis,  // ")"
    LeftCurlyBracket,  // "{"
    RightCurlyBracket, // "}"
    Plus,              // "+"
    Minus,             // "-"
    Or,                // "||"
    And,               // "&&"
    Asterisk,          // "*"
    Slash,             // "/"
    Not,               // "!"
    // Space,             // " "
}

impl Display for TokenGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenGroup::Keywords(word) => write!(f, "(0, {})", word.clone() as u8),
            TokenGroup::Delimiters(delims) => write!(f, "(1, {})", delims.clone() as u8),
            TokenGroup::Variables(vars) => write!(f, "(2, {})", vars),
            TokenGroup::Identifier(id) => write!(f, "(3, {})", id),
            TokenGroup::Eof => write!(f, "")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitType {
    Binary,
    Octal,
    Digital,
    HexPoint,
    Hex,
    Point,
}

use std::ops::Shr;

fn ordering(a: &DigitType, b: &DigitType) -> bool {
    let (a, b) = (a.clone() as u8, b.clone() as u8);
    if b >= a {
        true
    } else {
        false
    }
}

impl Shr for DigitType {
    type Output = bool;

    fn shr(self, other: Self) -> Self::Output {
        match self {
            DigitType::Point => match other {
                DigitType::Hex => false,
                _ => ordering(&self, &other)
            },
            DigitType::Hex => match other {
                DigitType::Point => false,
                _ => ordering(&self, &other)
            }
            _ => ordering(&self, &other)
        }
    }
}