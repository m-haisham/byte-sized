#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftAngle,
    RightAngle,
    Plus,
    Minus,
    Dot,
    Comma,
    LeftBracket,
    RightBracket,

    Error,
    EOF,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}
