#[derive(Debug, Clone)]
pub struct PrToken {
    pub kind: PrTokenKind,
    pub lexeme: String,
    pub line: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrTokenKind {
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
    // Colon,
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

impl PrToken {
    pub fn new(kind: PrTokenKind, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }

    pub fn empty() -> Self {
        Self {
            kind: PrTokenKind::Error,
            lexeme: String::from(""),
            line: 0,
        }
    }
}
