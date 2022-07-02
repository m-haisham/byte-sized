use super::token::{Token, TokenKind};

pub struct Scanner {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
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
            '<' => self.make_token(TokenKind::LeftAngle),
            '>' => self.make_token(TokenKind::RightAngle),
            '+' => self.make_token(TokenKind::Plus),
            '-' => self.make_token(TokenKind::Minus),
            '.' => self.make_token(TokenKind::Dot),
            ',' => self.make_token(TokenKind::Comma),
            '[' => self.make_token(TokenKind::LeftBracket),
            ']' => self.make_token(TokenKind::RightBracket),
            _ => self.error_token(),
        }
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
        Token::new(kind, self.line)
    }

    fn error_token(&self) -> Token {
        Token::new(TokenKind::Error, self.line)
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
    fn should_scan() {
        let mut scanner = Scanner::new(",[->+<]");
        let mut tokens = vec![];

        loop {
            let token = scanner.scan_token();
            tokens.push(token);

            match tokens.last().unwrap().kind {
                TokenKind::Error | TokenKind::EOF => break,
                _ => (),
            }
        }

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Comma, 1),
                Token::new(TokenKind::LeftBracket, 1),
                Token::new(TokenKind::Minus, 1),
                Token::new(TokenKind::RightAngle, 1),
                Token::new(TokenKind::Plus, 1),
                Token::new(TokenKind::LeftAngle, 1),
                Token::new(TokenKind::RightBracket, 1),
                Token::new(TokenKind::EOF, 1),
            ]
        );
    }
}
