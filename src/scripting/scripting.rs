use rlua::prelude::*;
use std::sync::{Mutex, MutexGuard};

// the main scripting resource
pub struct Scripting {
    lua: Mutex<Lua>,
}

impl Scripting {
    pub fn new() -> Self {
        Scripting {
            lua: Mutex::new(Lua::new())
        }
    }
    
    pub fn get(&self) -> MutexGuard<Lua> {
        self.lua.lock().unwrap()
    }
}