use std::{env, fs, process::{self, Command}};
use mlua::prelude::*;
use git2::{Repository, SubmoduleUpdateOptions};
use log::{error, info, warn};
use reqwest::Client;

use crate::util::{extract_zip, find_file, package_zip, prompt, is_tool};

pub fn run_lua(path: String) -> Result<(), LuaError> {
    let lua = Lua::new();
    let fs = lua.create_table().unwrap();
    let git = lua.create_table().unwrap();
    let os = lua.create_table().unwrap();
    let io = lua.create_table().unwrap();
    let http = lua.create_table().unwrap();
    let zip = lua.create_table().unwrap();
    let log = lua.create_table().unwrap();
    let globals = lua.globals();

    log.set("info", lua.create_function(move |_, msg: String| {
        info!("{}", msg);
        Ok(())
    })?).unwrap();

    log.set("warn", lua.create_function(move |_, msg: String| {
        warn!("{}", msg);
        Ok(())
    })?).unwrap();

    log.set("error", lua.create_function(move |_, msg: String| {
        error!("{}", msg);
        Ok(())
    })?).unwrap();

    fs.set("findfile", lua.create_function(move |_, path: String| {
        let path = find_file("src/.catalyst/", vec![path.as_str()]);
        match path {
            Ok(path) => Ok(path.display().to_string()),
            Err(_) => {
                error!("File not found.");
                Err(mlua::Error::external("File not found"))
            }
        }
    })?).unwrap();

    io.set("prompt", lua.create_function(move |_, msg: String| {
        match prompt(msg) {
            None => Ok(String::new()),
            Some(input) => {
                Ok(input.clone().trim().to_string())
            }
        }
    })?).unwrap();

    git.set("clonerepo", lua.create_function(move |_, (url, dest): (String, String)| {
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

    os.set("getenv", lua.create_function(move |_, key: String| {
        Ok(env::var(key).unwrap_or_default())
    })?).unwrap();

    os.set("setenv", lua.create_function(move |_, (key, value): (String, String)| {
        env::set_var(key, value);
        Ok(())
    })?).unwrap();

    os.set("name", lua.create_function(move |_, _: ()| {
        Ok(env::consts::OS)
    })?).unwrap();

    os.set("arch", lua.create_function(move |_, _: ()| {
        Ok(env::consts::ARCH)
    })?).unwrap();

    fs.set("getcwd", lua.create_function(move |_, _: ()| {
        Ok(env::current_dir().unwrap().display().to_string())
    })?).unwrap();

    fs.set("mkdir", lua.create_function(move |_, path: String| {
        Ok(fs::create_dir_all(path).unwrap())
    })?).unwrap();

    fs.set("exists", lua.create_function(move |_, path: String| {
        Ok(fs::metadata(path).is_ok())
    })?).unwrap();

    fs.set("readfile", lua.create_function(move |_, path: String| {
        let content = fs::read_to_string(path);
        match content {
            Ok(content) => Ok(content.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()),
            Err(err) => {
                error!("Failed to read file: {}", err);
                Err(mlua::Error::external("Failed to read file"))
            }
        }
    })?).unwrap();

    fs.set("writefile", lua.create_function(move |_, (path, content): (String, String)| {
        let _ = fs::write(path, content);
        Ok(())
    })?).unwrap();

    fs.set("readjson", lua.create_function(move |_, path: String| {
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

    fs.set("writejson", lua.create_function(move |_, (path, content): (String, String)| {
        let content = serde_json::to_string_pretty(&content).unwrap();
        let _ = fs::write(path, content);
        Ok(())
    })?).unwrap();

    git.set("submodulesinit", lua.create_function(move |_, _: ()| {
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

    http.set("fetch", lua.create_function(move |_, url: String| {
        let client = Client::new();

        let _ = client.get(url);
        Ok(())
    })?).unwrap();

    zip.set("zip", lua.create_function(move |_, (items, dest): (Vec<String>, String)| {
        package_zip(items, dest.as_str()).unwrap();
        Ok(())
    })?).unwrap();

    zip.set("unzip", lua.create_function(move |_, (file, dest): (String, String)| {
         extract_zip(file, dest).unwrap();
         Ok(())
    })?).unwrap();

    let _ = globals.set("isTool", lua.create_function(move |_, tool: String| {
        Ok(is_tool(tool.as_str()))
    })?);
    let _ = globals.set("fs", fs);
    let _ = globals.set("git", git);
    let _ = globals.set("os", os);
    let _ = globals.set("io", io);
    let _ = globals.set("http", http);
    let _ = globals.set("zip", zip);
    let _ = globals.set("log", log);

    let script_content = fs::read_to_string(path);

    match script_content {
        Err(err) => {
            error!("Failed to read script: {}", err);
            process::exit(1);
        }

        Ok(chunk) => {
            let script = chunk.lines().skip(1).collect::<Vec<_>>().join("\n");
            lua.load(script.as_str()).exec().expect("Failed to run script");
        }
    }
    Ok(())
}