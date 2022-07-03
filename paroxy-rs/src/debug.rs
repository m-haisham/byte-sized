use crate::{chunk::Chunk, opcode::OpCode};

#[cfg(feature = "debug")]
pub const DEBUG_PRINT_CODE: bool = true;

#[cfg(not(feature = "debug"))]
pub const DEBUG_PRINT_CODE: bool = false;

#[cfg(feature = "debug")]
pub const DEBUG_TRACE_EXECUTION: bool = true;

#[cfg(not(feature = "debug"))]
pub const DEBUG_TRACE_EXECUTION: bool = false;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:04} ");
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = match OpCode::try_from(chunk.code[offset]) {
        Ok(code) => code,
        Err(value) => {
            println!("Unknown opcode {value}");
            return offset + 1;
        }
    };

    match instruction {
        OpCode::DefineTape => simple_instruction("OP_DEFINE_TAPE", offset),
        OpCode::PointerValue => simple_instruction("OP_POINTER_VALUE", offset),
        OpCode::MoveLeft => simple_instruction("OP_MOVE_LEFT", offset),
        OpCode::MoveRight => simple_instruction("OP_MOVE_RIGHT", offset),
        OpCode::ShiftLeft => simple_instruction("OP_SHIFT_LEFT", offset),
        OpCode::ShiftRight => simple_instruction("OP_SHIFT_RIGHT", offset),
        OpCode::Increment => simple_instruction("OP_INCREMENT", offset),
        OpCode::Decrement => simple_instruction("OP_DECREMENT", offset),
        OpCode::IncrementSingular => simple_instruction("OP_INCREMENT_SINGLE", offset),
        OpCode::DecrementSingular => simple_instruction("OP_DECREMENT_SINGLE", offset),
        OpCode::Input => simple_instruction("OP_INPUT", offset),
        OpCode::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::Pop => simple_instruction("OP_POP", offset),
        OpCode::WriteString => simple_instruction("OP_WRITE_STRING", offset),
        OpCode::Print => simple_instruction("OP_PRINT", offset),
        OpCode::PrintRange => simple_instruction("OP_PRINT_RANGE", offset),
        OpCode::JumpIfFalse => jump_instruction("OP_JUMP_IF_FALSE", 1, chunk, offset),
        OpCode::Loop => jump_instruction("OP_LOOP", -1, chunk, offset),
        OpCode::Return => simple_instruction("OP_RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");

    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = &chunk.constants[constant as usize];
    println!("{name:16} {constant:4} {value:?}",);

    offset + 2
}

fn jump_instruction(name: &str, sign: i32, chunk: &Chunk, offset: usize) -> usize {
    let jump = ((chunk.code[offset + 1] as u16) << 8) | chunk.code[offset + 2] as u16;
    let dest = offset as i32 + 3 + sign * jump as i32;
    println!("{name:16} {offset:4} -> {dest}");

    offset + 3
}
