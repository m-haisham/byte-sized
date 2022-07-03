use std::io::stdin;

use crate::chunk::{Chunk, Value};
use crate::debug::{disassemble_instruction, DEBUG_TRACE_EXECUTION};
use crate::opcode::OpCode;

pub struct VM {
    chunk: Chunk,
    tape: Vec<u8>,
    ptr: usize,
    stack: Vec<Value>,
    ip: usize,
}

macro_rules! into_instruction {
    ($byte:expr) => {
        $byte.try_into().expect("Could not convert u8 into opcode.")
    };
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            tape: vec![],
            stack: vec![],
            ptr: 0,
            ip: 0,
        }
    }

    pub fn run(&mut self) {
        macro_rules! read_byte {
            () => {{
                self.ip += 1;
                self.chunk.code[self.ip - 1]
            }};
        }

        macro_rules! read_instruction {
            () => {
                into_instruction!(read_byte!())
            };
        }

        macro_rules! read_constant {
            () => {
                self.chunk.constants[read_byte!() as usize].clone()
            };
        }

        macro_rules! read_short {
            () => {{
                let a = read_byte!();
                let b = read_byte!();

                ((a as u16) << 8) | b as u16
            }};
        }

        macro_rules! ptr_deref {
            () => {
                self.tape[self.ptr]
            };
        }

        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for value in self.stack.iter() {
                    print!("[ {value} ]");
                }
                println!();

                disassemble_instruction(&self.chunk, self.ip);
            }

            let instruction: OpCode = read_instruction!();

            match instruction {
                OpCode::DefineTape => {
                    if let Value::Int(value) = self.stack_pop() {
                        self.tape.resize(value as usize, 0);
                    } else {
                        self.runtime_error("Expect an integer.");
                    }
                }
                OpCode::PointerValue => {
                    let value = ptr_deref!();
                    self.stack.push(Value::Int(value as u32));
                }
                OpCode::Constant => {
                    self.stack.push(read_constant!());
                }
                OpCode::MovePointerLeft => {
                    self.ptr -= 1;
                }
                OpCode::MovePointerRight => {
                    self.ptr += 1;
                }
                OpCode::IncrementSingular => {
                    ptr_deref!() += 1;
                }
                OpCode::DecrementSingular => {
                    ptr_deref!() -= 1;
                }
                OpCode::Print => {
                    println!("{}", self.stack_pop());
                }
                OpCode::Input => {
                    let mut line = String::new();
                    stdin().read_line(&mut line).unwrap();
                    match line.chars().nth(0) {
                        Some(char) => ptr_deref!() = char as u8,
                        None => (),
                    }
                }
                OpCode::JumpIfFalse => {
                    let offset = read_short!();
                    if !self.stack_peek(0).truthy() {
                        self.ip += offset as usize;
                    }
                }
                OpCode::Loop => {
                    let offset = read_short!();
                    self.ip -= offset as usize;
                }
                OpCode::Pop => {
                    self.stack_pop();
                }
                OpCode::Return => {
                    break;
                }
            }
        }
    }

    fn stack_pop(&mut self) -> Value {
        self.stack
            .pop()
            .take()
            .expect("Expect stack last item to be filled.")
    }

    fn stack_peek(&self, distance: usize) -> &Value {
        let index = (self.stack.len() - distance) - 1;
        &self.stack[index]
    }

    fn runtime_error(&mut self, message: &str) {
        eprintln!("{message}");
        self.stack.clear();
    }
}
