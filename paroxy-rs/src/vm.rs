use std::io::stdin;

use crate::chunk::{Chunk, Value};
use crate::debug::{disassemble_instruction, DEBUG_TRACE_EXECUTION};
use crate::opcode::OpCode;

pub struct VM {
    chunk: Chunk,
    tape: Vec<u8>,
    tape_size: usize,
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
            tape_size: 0,
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

        macro_rules! current_cell {
            () => {
                self.tape[self.ptr]
            };
        }

        loop {
            if DEBUG_TRACE_EXECUTION {
                let safe_cell = if self.tape.len() > self.ptr {
                    format!("{}->{}", self.ptr, self.tape[self.ptr])
                } else {
                    String::new()
                };

                print!("          ");
                print!("[{safe_cell}] ");
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
                        self.tape_size = value as usize;
                    } else {
                        self.runtime_error("Expect an integer.");
                    }
                }
                OpCode::PointerValue => {
                    let value = current_cell!();
                    self.stack.push(Value::Int(value as u32));
                }
                OpCode::SetPointer => {
                    let value = self.stack_pop();
                    if let Value::Int(value) = value {
                        if (value as usize) < self.tape.len() {
                            self.ptr = value as usize;
                        } else {
                            self.runtime_error("Tape overflow.");
                        }
                    } else {
                        self.runtime_error("Expect an integer.");
                    }
                }
                OpCode::Constant => {
                    self.stack.push(read_constant!());
                }
                OpCode::MoveLeft => {
                    let value = self.stack_pop();
                    if let Value::Int(value) = value {
                        if self.ptr > value as usize {
                            self.ptr -= value as usize;
                        } else {
                            self.runtime_error("Pointer cannot move below zero.");
                        }
                    } else {
                        self.runtime_error("Expect an integer.");
                    }
                }

                OpCode::MoveRight => {
                    let value = self.stack_pop();
                    if let Value::Int(value) = value {
                        if (self.ptr + value as usize) <= self.tape_size {
                            self.ptr += value as usize;
                        } else {
                            self.runtime_error("Pointer exceeds tape size.");
                        }
                    } else {
                        self.runtime_error("Expect an integer.");
                    }
                }
                OpCode::ShiftLeft => {
                    self.ptr -= 1;
                }
                OpCode::ShiftRight => {
                    self.ptr += 1;
                }
                OpCode::Increment => {
                    let value = read_byte!();
                    let available = u8::MAX - current_cell!();
                    if available > value {
                        current_cell!() += value as u8;
                    } else {
                        self.runtime_error(
                            format!(
                                "Cannot be greater than {} [{}]",
                                u8::MAX,
                                value + current_cell!()
                            )
                            .as_str(),
                        );
                    }
                }
                OpCode::Decrement => {
                    let value = read_byte!();
                    let available = current_cell!() - u8::MIN;
                    if available > value {
                        current_cell!() -= value as u8;
                    } else {
                        self.runtime_error(
                            format!(
                                "Cannot be less than {} [{}]",
                                u8::MIN,
                                value - current_cell!()
                            )
                            .as_str(),
                        );
                    }
                }
                OpCode::IncrementSingular => {
                    current_cell!() += 1;
                }
                OpCode::DecrementSingular => {
                    current_cell!() -= 1;
                }
                OpCode::WriteString => {
                    let value = self.stack_pop();
                    if let Value::String(value) = value {
                        for (i, c) in value.chars().enumerate() {
                            self.tape[self.ptr + i] = c as u8;
                        }
                    } else {
                        self.runtime_error("Expect a string value.");
                    }
                }
                OpCode::WriteCell => {
                    current_cell!() = read_byte!();
                }
                OpCode::Print => {
                    print!("{}", current_cell!() as char);

                    // Make sure stack trace is on the next line.
                    if DEBUG_TRACE_EXECUTION {
                        println!();
                    }
                }
                OpCode::PrintRange => {
                    let value = self.stack_pop();
                    if let Value::Int(value) = value {
                        let range = &self.tape[self.ptr..self.ptr + value as usize];
                        let output = range.iter().map(|c| *c as char).collect::<String>();
                        print!("{output}");

                        // Make sure stack trace is on the next line.
                        if DEBUG_TRACE_EXECUTION {
                            println!();
                        }
                    } else {
                        self.runtime_error("Expect a number.");
                    }
                }
                OpCode::Input => {
                    let mut line = String::new();
                    stdin().read_line(&mut line).unwrap();
                    match line.chars().nth(0) {
                        Some(char) => current_cell!() = char as u8,
                        None => (),
                    }
                }
                OpCode::JumpIfZero => {
                    let offset = read_short!();
                    if current_cell!() == 0 {
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
