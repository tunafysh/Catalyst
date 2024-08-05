use std::fs;
use colored::Colorize;
use mlua::prelude::*;

pub fn run_script(path: String) -> Result<(), LuaError> {
    let lua = Lua::new();
    let _ = lua.create_function(|printtest, ()| Ok({
        println!("Hello, Lua!")
    }));
    let script_content = fs::read_to_string(path)?;
    lua.load(script_content.as_str()).exec()?;
    Ok(())
}