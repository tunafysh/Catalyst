use std::{fs, process};
use mlua::prelude::*;
use log::{info, error};

pub fn run_script(path: String) -> Result<(), LuaError> {
    let lua = Lua::new();
    let script_content = fs::read_to_string(path);
    match script_content {
        Err(err) => {
            error!("Failed to read script: {}", err);
            process::exit(1);
        }

        Ok(chunk) => {
            info!("The function is running");
            lua.load(chunk.as_str()).exec().expect("Failed to run script");
        }
    }
    Ok(())
}