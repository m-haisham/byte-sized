use std::{collections::VecDeque, fs};

use rlua::{Function, Lua, Table, Value};

pub struct Scripts {
    pub lua: Lua,
    pub history: Vec<HistoryEvent>,
}

#[derive(Debug)]
pub struct HistoryEvent {
    pub snapshot: Vec<f32>,
    pub accesses: Vec<usize>,
    pub writes: Vec<usize>,
}

impl Scripts {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            history: Vec::new(),
        }
    }

    pub fn load_lib(self) -> Self {
        self.lua.context(|lua_ctx| {
            let lib = fs::read_to_string("lib/lib.lua").unwrap();
            lua_ctx.load(&lib).exec().unwrap();
        });
        self
    }

    pub fn run_algorithm(&mut self, values: Vec<f32>) {
        self.history.clear();

        self.lua.context(|lua_ctx| {
            let sequence_table = lua_ctx.create_table().unwrap();
            sequence_table.set("inner", values).unwrap();

            let history = lua_ctx.create_table().unwrap();
            sequence_table.set("history", history).unwrap();

            let globals = lua_ctx.globals();

            let execute: Function = globals.get("execute").unwrap();
            let history = match execute.call::<_, (Vec<Table>,)>(("quicksort", sequence_table)) {
                Err(e) => {
                    println!("{}", e);
                    panic!();
                }
                Ok(value) => value.0,
            };

            self.history
                .extend(history.into_iter().map(Self::parse_event));

            // println!("{:?}", self.history);
        });
    }

    fn parse_event(table: Table) -> HistoryEvent {
        HistoryEvent {
            snapshot: table.get("snapshot").unwrap(),
            accesses: table.get("accesses").unwrap(),
            writes: table.get("writes").unwrap(),
        }
    }
}
