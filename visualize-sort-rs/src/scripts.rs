use std::fs;

use rlua::{Function, Lua, Table, Value};

pub struct Scripts {
    pub lua: Lua,
}

impl Scripts {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    pub fn load_lib(self) -> Self {
        self.lua.context(|lua_ctx| {
            let lib = fs::read_to_string("scripts/lib.lua").unwrap();
            lua_ctx.load(&lib).exec().unwrap();
        });
        self
    }

    pub fn run_algorithm(&self, values: Vec<f32>) {
        self.lua.context(|lua_ctx| {
            let sequence_table = lua_ctx.create_table().unwrap();
            sequence_table.set("inner", values).unwrap();

            let history = lua_ctx.create_table().unwrap();
            sequence_table.set("history", history).unwrap();

            let globals = lua_ctx.globals();

            let execute: Function = globals.get("execute").unwrap();
            if let Err(e) = execute.call::<_, ()>(("quicksort", sequence_table)) {
                println!("{}", e);
                panic!();
            };
        });
    }
}
