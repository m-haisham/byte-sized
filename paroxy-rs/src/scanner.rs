use super::token::{Token, TokenKind};

pub struct Scanner<'a> {
    source: &'a str,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::EOF);
        }

        let c = self.advance();

        match c {
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            '[' => self.make_token(TokenKind::LeftBracket),
            ']' => self.make_token(TokenKind::RightBracket),
            '<' => self.make_token(TokenKind::LeftAngle),
            '>' => self.make_token(TokenKind::RightAngle),
            '.' => self.make_token(TokenKind::Dot),
            ',' => self.make_token(TokenKind::Comma),
            '$' => self.make_token(TokenKind::Dollar),
            '@' => self.make_token(TokenKind::At),
            '#' => self.make_token(TokenKind::Hash),
            '*' => self.make_token(TokenKind::Star),
            '^' => self.make_token(TokenKind::Caret),
            '+' => self.make_token(TokenKind::Plus),
            '-' => self.make_token(TokenKind::Minus),
            n @ ('\'' | '"') => self.string(n),
            n @ _ => {
                if self.is_digit(n) {
                    return self.integer();
                }

                return self.make_token(TokenKind::Ignore);
            }
        }
    }

    fn string(&mut self, terminator: char) -> Token {
        while !self.is_at_end() && self.peek() != terminator {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        self.make_token(TokenKind::String)
    }

    fn integer(&mut self) -> Token {
        while !self.is_at_end() && self.is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(TokenKind::Integer)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    fn peek(&self) -> char {
        self.chars[self.current]
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, String::from(lexeme), self.line)
    }

    fn error_token(&self, message: &'static str) -> Token {
        Token::new(TokenKind::Error, String::from(message), self.line)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            match self.peek() {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance()
                }
                _ => return,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_scan_brainxysm() {
        let mut scanner = Scanner::new("{30000}\"Hello world!\n\"$");
        let mut tokens = vec![];

        loop {
            let token = scanner.scan_token();
            tokens.push(token.kind);

            match tokens.last().unwrap() {
                TokenKind::Error | TokenKind::EOF => break,
                _ => (),
            }
        }

        assert_eq!(
            tokens,
            vec![
                TokenKind::LeftBrace,
                TokenKind::Integer,
                TokenKind::RightBrace,
                TokenKind::String,
                TokenKind::Dollar,
                TokenKind::EOF,
            ]
        );
    }
}
