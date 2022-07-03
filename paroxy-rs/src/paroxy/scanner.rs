use super::token::{PrToken, PrTokenKind};

pub struct PrScanner<'a> {
    source: &'a str,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> PrScanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> PrToken {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(PrTokenKind::EOF);
        }

        let c = self.advance();

        match c {
            '{' => self.make_token(PrTokenKind::LeftBrace),
            '}' => self.make_token(PrTokenKind::RightBrace),
            '[' => self.make_token(PrTokenKind::LeftBracket),
            ']' => self.make_token(PrTokenKind::RightBracket),
            '<' => self.make_token(PrTokenKind::LeftAngle),
            '>' => self.make_token(PrTokenKind::RightAngle),
            '.' => self.make_token(PrTokenKind::Dot),
            ',' => self.make_token(PrTokenKind::Comma),
            '$' => self.make_token(PrTokenKind::Dollar),
            '@' => self.make_token(PrTokenKind::At),
            '#' => self.make_token(PrTokenKind::Hash),
            ':' => self.make_token(PrTokenKind::Colon),
            '+' => self.make_token(PrTokenKind::Plus),
            '-' => self.make_token(PrTokenKind::Minus),
            n @ ('\'' | '"') => self.string(n),
            n @ _ => {
                if self.is_digit(n) {
                    return self.integer();
                }

                return self.error_token("Unexpected character.");
            }
        }
    }

    fn string(&mut self, terminator: char) -> PrToken {
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
        self.make_token(PrTokenKind::String)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn integer(&mut self) -> PrToken {
        while !self.is_at_end() && self.is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(PrTokenKind::Integer)
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

    fn make_token(&self, kind: PrTokenKind) -> PrToken {
        let lexeme = &self.source[self.start..self.current];
        PrToken::new(kind, String::from(lexeme), self.line)
    }

    fn error_token(&self, message: &'static str) -> PrToken {
        PrToken::new(PrTokenKind::Error, String::from(message), self.line)
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
        let mut scanner = PrScanner::new("{30000}\"Hello world!\n\"$");
        let mut tokens = vec![];

        loop {
            let token = scanner.scan_token();
            tokens.push(token.kind);

            match tokens.last().unwrap() {
                PrTokenKind::Error | PrTokenKind::EOF => break,
                _ => (),
            }
        }

        assert_eq!(
            tokens,
            vec![
                PrTokenKind::LeftBrace,
                PrTokenKind::Integer,
                PrTokenKind::RightBrace,
                PrTokenKind::String,
                PrTokenKind::Dollar,
                PrTokenKind::EOF,
            ]
        );
    }
}
