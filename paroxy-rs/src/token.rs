#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Single character tokens.
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    Comma,
    Dollar,
    At,
    Hash,
    Dot,
    Star,
    Caret,
    Plus,
    Minus,

    // Literals.
    Integer,
    String,

    // Misc.
    Ignore,
    Error,
    EOF,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }

    pub fn empty() -> Self {
        Self {
            kind: TokenKind::Error,
            lexeme: String::from(""),
            line: 0,
        }
    }
}
