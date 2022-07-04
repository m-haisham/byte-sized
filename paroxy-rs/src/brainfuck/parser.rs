use std::mem;

use crate::{
    chunk::{Chunk, Value},
    debug::{disassemble_chunk, DEBUG_PRINT_CODE},
    opcode::OpCode,
};

use super::{
    scanner::BfScanner,
    token::{Token, TokenKind},
};

pub struct BfParser<'a> {
    scanner: BfScanner,
    chunk: &'a mut Chunk,

    previous: Token,
    current: Token,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> BfParser<'a> {
    pub fn new(scanner: BfScanner, chunk: &'a mut Chunk) -> Self {
        Self {
            scanner,
            chunk,
            previous: Token::new(TokenKind::Error, 0),
            current: Token::new(TokenKind::Error, 0),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(&mut self) -> bool {
        self.emit_constant(Value::Int(30000));
        self.emit_byte(OpCode::DefineTape);

        self.advance();
        while !self.matches(TokenKind::EOF) {
            self.expression();
        }

        self.end()
    }

    pub fn expression(&mut self) {
        match &self.current.kind {
            TokenKind::LeftAngle => self.emit_byte(OpCode::ShiftLeft as u8),
            TokenKind::RightAngle => self.emit_byte(OpCode::ShiftRight as u8),
            TokenKind::Plus => self.emit_byte(OpCode::IncrementSingular as u8),
            TokenKind::Minus => self.emit_byte(OpCode::DecrementSingular as u8),
            TokenKind::Dot => self.emit_byte(OpCode::Print as u8),
            TokenKind::Comma => self.emit_byte(OpCode::Input as u8),
            TokenKind::LeftBracket => self.repeat(),
            _ => panic!("Unexpected token"),
        }

        self.advance();
    }

    pub fn repeat(&mut self) {
        let loop_start = self.current_chunk().code.len();
        let repeat_jump = self.emit_jump(OpCode::JumpIfZero);

        self.advance();
        while !self.check(TokenKind::RightBracket) {
            self.expression();
        }

        self.emit_loop(loop_start);
        self.patch_jump(repeat_jump);
    }

    fn advance(&mut self) {
        mem::swap(&mut self.current, &mut self.previous);

        loop {
            self.current = self.scanner.scan_token();

            match self.current.kind {
                TokenKind::Error => (),
                _ => break,
            }

            self.error_at_current("unexpected token.");
        }
    }

    fn matches(&mut self, kind: TokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous.clone(), message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current.clone(), message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.kind {
            TokenKind::EOF => eprint!(" at end"),
            TokenKind::Error => (),
            _ => (),
        }

        println!(": {message}");
        self.had_error = true;
    }

    fn emit_byte<T: Into<u8>>(&mut self, byte: T) {
        let line = self.previous.line;
        self.current_chunk().write_chunk(byte.into(), line);
    }

    fn emit_two_bytes<T: Into<u8>>(&mut self, byte1: T, byte2: T) {
        let line = self.previous.line;
        self.current_chunk().write_chunk(byte1.into(), line);
        self.current_chunk().write_chunk(byte2.into(), line);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_two_bytes(OpCode::Constant as u8, constant);
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction as u8);
        self.emit_byte(0xff);
        self.emit_byte(0xff);

        self.current_chunk().code.len() - 2
    }

    fn patch_jump(&mut self, offset: usize) {
        // -2 to adjust for the bytecode for the jump offset itself
        let jump = self.current_chunk().code.len() - offset - 2;

        if jump > u16::MAX as usize {
            self.error("Too much code to jump over.");
        }

        let [a, b] = (jump as u16).to_be_bytes();

        self.current_chunk().code[offset] = a;
        self.current_chunk().code[offset + 1] = b;
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::Loop as u8);

        let offset = self.current_chunk().code.len() - loop_start + 2;
        if offset > u16::MAX as usize {
            self.error("Loop body too large.");
        }

        let [a, b] = (offset as u16).to_be_bytes();

        self.emit_byte(a);
        self.emit_byte(b);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > (u8::MAX as usize) {
            self.error("Too many constants in one chunk.");
        }

        constant as u8
    }

    fn end(&mut self) -> bool {
        self.emit_return();

        if DEBUG_PRINT_CODE {
            disassemble_chunk(self.current_chunk(), "<script>");
        }

        !self.had_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_brainfuck() {
        let scanner = BfScanner::new(",[->+<].");
        let mut chunk = Chunk::new();

        let mut parser = BfParser::new(scanner, &mut chunk);
        parser.compile();

        assert_eq!(
            chunk.code,
            vec![
                OpCode::Constant as u8,
                OpCode::DefineTape as u8,
                0,
                OpCode::Input as u8,
                OpCode::PointerValue as u8,
                OpCode::JumpIfZero as u8,
                0,
                8,
                OpCode::Pop as u8,
                OpCode::DecrementSingular as u8,
                OpCode::ShiftRight as u8,
                OpCode::IncrementSingular as u8,
                OpCode::ShiftLeft as u8,
                OpCode::Loop as u8,
                0,
                12,
                OpCode::Pop as u8,
                OpCode::PointerValue as u8,
                OpCode::Print as u8,
                OpCode::PointerValue as u8,
                OpCode::Return as u8,
            ],
        );
    }
}
