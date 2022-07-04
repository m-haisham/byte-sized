use std::{mem, rc::Rc};

use crate::{
    chunk::{Chunk, Value},
    debug::{disassemble_chunk, DEBUG_PRINT_CODE},
    opcode::OpCode,
};

use super::{
    scanner::PrScanner,
    token::{PrToken, PrTokenKind},
};

pub struct PrParser<'a> {
    scanner: PrScanner<'a>,
    chunk: &'a mut Chunk,
    previous: PrToken,
    current: PrToken,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> PrParser<'a> {
    pub fn new(scanner: PrScanner<'a>, chunk: &'a mut Chunk) -> Self {
        Self {
            scanner,
            chunk,
            previous: PrToken::empty(),
            current: PrToken::empty(),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(&mut self) -> bool {
        self.advance();

        // Default tape definition
        if self.current.kind != PrTokenKind::LeftBracket {
            self.emit_constant(Value::Int(30000));
            self.emit_byte(OpCode::DefineTape);
        }

        while !self.matches(PrTokenKind::EOF) {
            self.expression();
        }

        self.end()
    }

    pub fn expression(&mut self) {
        match &self.current.kind {
            PrTokenKind::Plus => self.sized_code(OpCode::IncrementSingular, OpCode::Increment),
            PrTokenKind::Minus => self.sized_code(OpCode::DecrementSingular, OpCode::Decrement),
            PrTokenKind::LeftAngle => self.sized_constant(OpCode::ShiftLeft, OpCode::MoveLeft),
            PrTokenKind::RightAngle => self.sized_constant(OpCode::ShiftRight, OpCode::MoveRight),
            PrTokenKind::Dot => self.sized_constant(OpCode::Print, OpCode::PrintRange),
            PrTokenKind::Comma => self.input_expression(),
            PrTokenKind::Hash => self.replace_current(),
            PrTokenKind::At => self.set_pointer_expression(),
            PrTokenKind::LeftBrace => self.define_tape(),
            PrTokenKind::LeftBracket => self.loop_expression(),
            PrTokenKind::String => self.string(),
            _ => (),
        }
    }

    fn sized_constant(&mut self, one: OpCode, many: OpCode) {
        self.advance();
        if self.matches(PrTokenKind::Integer) {
            let size = self.previous.lexeme.parse::<u32>().unwrap();
            self.emit_constant(Value::Int(size));
            self.emit_byte(many);
        } else {
            self.emit_byte(one);
        }
    }

    fn sized_code(&mut self, one: OpCode, many: OpCode) {
        self.advance();
        if self.matches(PrTokenKind::Integer) {
            let size = self.previous.lexeme.parse::<usize>().unwrap();

            if size > u8::MAX as usize {
                self.error_at_current("Expect integer between 0-255.");
                return;
            }

            self.emit_byte(many);
            self.emit_byte(size as u8);
        } else {
            self.emit_byte(one);
        }
    }

    fn input_expression(&mut self) {
        self.advance();
        self.emit_byte(OpCode::Input);
    }

    fn replace_current(&mut self) {
        self.advance();

        self.consume(PrTokenKind::Integer, "Expect integer after '#'.");
        let value = self.previous.lexeme.parse::<usize>().unwrap();
        if value > u8::MAX as usize {
            self.error_at(
                self.previous.clone(),
                "Expect integer between 0 and 255 (included).",
            );
            return;
        }

        self.emit_byte(OpCode::WriteCell);
        self.emit_byte(value as u8);
    }

    fn set_pointer_expression(&mut self) {
        self.advance();

        self.consume(PrTokenKind::Integer, "Expect integer after '@'.");
        let value = self.previous.lexeme.parse::<u32>().unwrap();

        self.emit_constant(Value::Int(value));
        self.emit_byte(OpCode::SetPointer);
    }

    fn define_tape(&mut self) {
        self.advance();
        self.consume(PrTokenKind::Integer, "Expect a number after '{'.");
        let size = self.previous.lexeme.parse::<u32>().unwrap();

        self.emit_constant(Value::Int(size));
        self.emit_byte(OpCode::DefineTape);

        self.consume(PrTokenKind::RightBrace, "Expect '}' after define tape.");
    }

    fn loop_expression(&mut self) {
        let loop_start = self.current_chunk().code.len();
        let repeat_jump = self.emit_jump(OpCode::JumpIfZero);

        self.advance();
        while !self.matches(PrTokenKind::RightBracket) {
            self.expression();
        }

        self.emit_loop(loop_start);
        self.patch_jump(repeat_jump);
    }

    pub fn string(&mut self) {
        let value = String::from(&self.current.lexeme[1..self.current.lexeme.len() - 1]);
        let length = value.len();

        let rc = Rc::from(value);

        self.emit_constant(Value::String(rc));
        self.emit_byte(OpCode::WriteString);
        self.advance();

        if self.matches(PrTokenKind::Dollar) {
            self.emit_constant(Value::Int(length as u32));
            self.emit_byte(OpCode::PrintRange);
        }
    }

    fn advance(&mut self) {
        mem::swap(&mut self.current, &mut self.previous);

        loop {
            self.current = self.scanner.scan_token();

            match self.current.kind {
                PrTokenKind::Error => (),
                PrTokenKind::Ignore => continue,
                _ => break,
            }

            self.error_at_current("unexpected token.");
        }
    }

    fn matches(&mut self, kind: PrTokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&self, kind: PrTokenKind) -> bool {
        self.current.kind == kind
    }

    fn consume(&mut self, kind: PrTokenKind, message: &str) {
        if self.current.kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous.clone(), message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current.clone(), message);
    }

    fn error_at(&mut self, token: PrToken, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.kind {
            PrTokenKind::EOF => eprint!(" at end"),
            PrTokenKind::Error => (),
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
