use std::{env, fs, process::{self, Command}};
use clap::builder::Str;
use mlua::prelude::*;
use git2::{Repository, SubmoduleUpdateOptions};
use log::{error, info, warn};
use reqwest::Client;

use crate::util::{self, compile_all, extract_zip, find_file, package_zip, prompt};

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
        match path {
            Ok(path) => Ok(path.display().to_string()),
            Err(_) => {
                error!("File not found.");
                Err(mlua::Error::external("File not found"))
            }
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

    globals.set("shell", lua.create_function(move |_, (shell, command): (String, String)| {
        Command::new(shell)
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process");

        Ok(())
    })?).unwrap();

    globals.set("getenv", lua.create_function(move |_, key: String| {
        Ok(env::var(key).unwrap_or_default())
    })?).unwrap();

    globals.set("setenv", lua.create_function(move |_, (key, value): (String, String)| {
        env::set_var(key, value);
        Ok(())
    })?).unwrap();

    globals.set("getcwd", lua.create_function(move |_, _: ()| {
        Ok(env::current_dir().unwrap().display().to_string())
    })?).unwrap();

    globals.set("exists", lua.create_function(move |_, path: String| {
        Ok(fs::metadata(path).is_ok())
    })?).unwrap();

    globals.set("readfile", lua.create_function(move |_, path: String| {
        let content = fs::read_to_string(path);
        match content {
            Ok(content) => Ok(content),
            Err(err) => {
                error!("Failed to read file: {}", err);
                Err(mlua::Error::external("Failed to read file"))
            }
        }
    })?).unwrap();

    globals.set("writefile", lua.create_function(move |_, (path, content): (String, String)| {
        let _ = fs::write(path, content);
        Ok(())
    })?).unwrap();

    globals.set("readjson", lua.create_function(move |_, path: String| {
        let content = fs::read_to_string(path);
        match content {
            Ok(content) => {
                let json: serde_json::Value = serde_json::from_str(&content).unwrap();
                Ok(json.to_string())
            },
            Err(err) => {
                error!("Failed to read file: {}", err);
                Err(mlua::Error::external("Failed to read file"))
            }
        }
    })?).unwrap();

    globals.set("writejson", lua.create_function(move |_, (path, content): (String, String)| {
        let content = serde_json::to_string_pretty(&content).unwrap();
        let _ = fs::write(path, content);
        Ok(())
    })?).unwrap();

    globals.set("submodulesinit", lua.create_function(move |_, _: ()| {
        let repo = Repository::open("/path/to/your/repo").unwrap();

    // Initialize and update submodules
    while let Ok(submodule) = repo.submodules() {
        for mut submod in submodule {
            let _ = submod.init(true);

        // Update the submodule
            let mut options = SubmoduleUpdateOptions::new();
            let _ = submod.update(true, Some(&mut options));
        }
    }

    Ok(())
    })?).unwrap();

    globals.set("fetch", lua.create_function(move |_, url: String| {
        let client = Client::new();

        let _ = client.get(url);
        Ok(())
    })?).unwrap();

    globals.set("zip", lua.create_function(move |_, (items, dest): (Vec<&str>, String)| {
        package_zip(items, dest.as_str()).unwrap();
        Ok(())
    })?).unwrap();

    globals.set("unzip", lua.create_function(move |_, (file, dest): (String, String)| {
         extract_zip(file, dest).unwrap();
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