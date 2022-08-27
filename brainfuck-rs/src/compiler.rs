use std::{iter::Peekable, str::Chars};

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    IncPtr(u32),
    DecPtr(u32),
    IncData(u8),
    DecData(u8),
    Input,
    Output,
    Loop(Vec<Expr>),
}

pub struct Compiler<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }

    pub fn compile(&mut self) -> Vec<Expr> {
        let mut tokens = vec![];

        self.skip_loop();

        while let Some(c) = self.chars.next() {
            self.compile_token(&mut tokens, c);
        }

        tokens
    }

    fn skip_loop(&mut self) {
        if let Some('[') = self.chars.peek() {
            self.chars.next();
            let mut depth = 1usize;

            while depth > 0 {
                match self.chars.next() {
                    Some('[') => depth += 1,
                    Some(']') => {
                        // FIXME: Prevent underflow;
                        depth -= 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn compile_token(&mut self, tokens: &mut Vec<Expr>, c: char) {
        macro_rules! count_chars {
            ($c:expr) => {{
                let mut value = 1;
                while matches!(self.chars.peek(), Some($c)) {
                    value += 1;
                    self.chars.next();
                }
                value
            }};
        }

        let token = match c {
            '>' => Expr::IncPtr(count_chars!('>')),
            '<' => Expr::DecPtr(count_chars!('<')),
            '+' => Expr::IncData((count_chars!('+') % 256) as u8),
            '-' => Expr::DecData((count_chars!('-') % 256) as u8),
            '.' => Expr::Output,
            ',' => Expr::Input,
            '[' => {
                let mut loop_tokens = vec![];
                loop {
                    match self.chars.peek() {
                        Some('[') | None => break,
                        Some(_) => {
                            // call next and unwrap peeked value to avoid copying.
                            let c = self.chars.next().unwrap();
                            self.compile_token(&mut loop_tokens, c);
                        }
                    }
                }

                Expr::Loop(loop_tokens)
            }
            _ => return,
        };

        tokens.push(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compile() {
        let source = "+[->+<]";
        let tokens = Compiler::new(source.chars()).compile();

        use Expr::*;
        assert_eq!(
            tokens,
            vec![
                IncData(1),
                Loop(vec![DecData(1), IncPtr(1), IncData(1), DecPtr(1)])
            ]
        )
    }
}
