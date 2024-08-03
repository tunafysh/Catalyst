use std::{fs, process};
use colored::Colorize;
use mlua::prelude::*;

pub fn run_script(path: String) -> Result<(), LuaError> {
    let lua = Lua::new();
    let _ = lua.create_function(|_, ()| Ok({
        println!("Hello, Lua!");
    }));
    let script_content = fs::read_to_string(path);
    if let Ok(content) = script_content {
        lua.load(content.as_str()).exec()
    } else {
        println!("{}", "Cannot read script file.".to_string().red());
        process::exit(5);
    }
}