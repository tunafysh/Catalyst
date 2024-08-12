use std::{fs, process};
use mlua::prelude::*;
use git2::{Repository, SubmoduleUpdate, Submodule};
use log::{error, info, warn};

use crate::util::{compile_all, find_file, prompt};

pub fn run_script(path: String) -> Result<(), LuaError> {
    let lua = Lua::new();
    let globals = lua.globals();

    globals.set("log", lua.create_function(move |_, msg: String| {
        info!("{}", msg);
        Ok(())
    })?).unwrap();

    globals.set("warn", lua.create_function(move |_, msg: String| {
        warn!("{}", msg);
        Ok(())
    })?).unwrap();

    globals.set("error", lua.create_function(move |_, msg: String| {
        error!("{}", msg);
        Ok(())
    })?).unwrap();

    globals.set("compileall", lua.create_function(move |_, _: ()| {
        compile_all();
        Ok(())
    })?).unwrap();

    globals.set("findfile", lua.create_function(move |_, path: String| {
        let path = find_file("src/.catalyst/", vec![path.as_str()]);
        if let Some(path) = path {
            Ok(path.display().to_string())
        }
        else {
            Ok("".to_string())
        }
    })?).unwrap();

    globals.set("prompt", lua.create_function(move |_, msg: String| {
        match prompt(msg) {
            None => Ok(String::new()),
            Some(input) => {
                Ok(input.clone().trim().to_string())
            }
        }
    })?).unwrap();

    globals.set("clonerepo", lua.create_function(move |_, (url, dest): (String, String)| {
        let repo = Repository::clone(url.as_str(), dest);
        match repo {
            Ok(_) => {},
            Err(err) => {
                error!("Failed to clone repository: {}", err);
                process::exit(1);
            }
        };
        Ok(())
    })?).unwrap();

    globals.set("submoduleinit", lua.create_function(move |_, path: String| {
        warn!("under construction");
        Ok(())
    })?).unwrap();
    let script_content = fs::read_to_string(path);
    match script_content {
        Err(err) => {
            error!("Failed to read script: {}", err);
            process::exit(1);
        }

        Ok(chunk) => {
            lua.load(chunk.as_str()).exec().expect("Failed to run script");
        }
    }
    Ok(())
}