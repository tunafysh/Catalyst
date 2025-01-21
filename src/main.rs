use std::{env::consts, fs, path::Path, process};
use glob::glob;
use owo_colors::{OwoColorize, Stream::Stdout};
use log::{error, info};
use util::find_file;

mod structs;
mod util;
mod logger;
mod debug;
mod updater;
mod lua;
mod jscript;

const CATALYST_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = util::args();
    if !matches.get_flag("nologs") {
        match logger::setup_logger(matches.clone()) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", format!("Failed to setup logger. Error: {}", err).to_string().if_supports_color(Stdout, |text| text.red()));
                process::exit(1);
            }
        }
    }

    util::banner(matches.clone());

    match matches.subcommand() {
        Some(("init", _)) => {
            info!("Initializing configuration...");
            let gensuccess = util::generate();
            process::exit(gensuccess as i32);
        }
        Some(("update", _)) => {
            updater::update(CATALYST_VERSION);
        }

        Some(("check", _)) => {
            updater::check(CATALYST_VERSION);
        }
        Some(("cleanup", _)) => {
            let logdir = if consts::OS == "windows" {
                Path::new("C:\\Users\\%USERNAME%\\AppData\\Local\\Temp\\Catalyst")
            } else {
                Path::new("~/.catalyst/cache")
            };
            if Path::exists(logdir) {
                match fs::remove_dir_all(logdir) {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Failed to remove logs directory");
                    }
                }
            }
        }
        _ => {}
    }

    if matches.get_flag("debug") {
        debug::debug();
    }


    let hook = matches.get_many::<String>("hook").unwrap_or_default().into_iter().map(|v| v.as_str()).collect::<Vec<_>>();
    if !hook.is_empty() {
        let script = fs::read_to_string(hook[0]).unwrap();
        let first_line = script.lines().next().unwrap_or("").to_string();
        info!("{}", first_line);
        if first_line == "\"use js\"" {
            let _ = jscript::run_js(script);
        }
        else if first_line == "\"use lu\"a" {
            let _ = lua::run_lua(script);
        }
        else {
            error!("Invalid hook file");
        }
    }
    else{

        info!("Detecting hooks...");
        let hooks = glob("**/*.cly").unwrap();

    info!("Running hooks...");
    for hook in hooks {
        match hook {
            Ok(hook) => {

                let hook = hook.display().to_string();
                info!("{}", format!("Running hook: {}", hook).to_string().if_supports_color(Stdout, |text| text.cyan()));
                match fs::read_to_string(&hook) {
                    Ok(content) => {
                        let first_line = content.lines().next().unwrap_or("").to_string();
                        info!("{}", first_line);
                        if first_line == "use js" {
                            let _ = jscript::run_js(content);
                        }
                        else if first_line == "use lua" {
                            let _ = lua::run_lua(content);
                        }
                        else {
                            error!("Invalid hook file");
                        }
                        
                    },
                    Err(_) => {
                        error!("Failed to read hook file");
                        continue;
                    }
                };
            }
            Err(_) => {
                error!("Failed to run hook");
            }
        }
    }
    }
}