use std::{fmt::Display, rc::Rc};

use bincode::{DefaultOptions, Options};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Int(u32),
    String(Rc<str>),
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn write_chunk(&mut self, value: u8, line: usize) {
        self.code.push(value);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        DefaultOptions::new().with_varint_encoding().serialize(self)
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, Box<bincode::ErrorKind>> {
        DefaultOptions::new()
            .with_varint_encoding()
            .deserialize(bytes)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "{value}"),
        }
    }
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Int(value) => *value != 0,
            Value::String(value) => true,
        }
    }
}
