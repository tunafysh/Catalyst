use std::{fs, io::{self, Write}, path::Path};
use serde_json::to_string;
use clap::ArgMatches;
use colored::Colorize;

use crate::structs;

pub fn find_file<P: AsRef<Path>>(dir: P, file_name: &str) -> Option<std::path::PathBuf> {
    for entry in fs::read_dir(dir).expect("Directory not found") {
        let entry = entry.expect("Unable to read entry");
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == file_name {
            return Some(path);
        }
    }
    None
}

pub fn verbose(matches: ArgMatches, msg: String) {
    if matches.get_flag("verbose") {
        println!("{}", msg.cyan());
    }
}

pub fn generate() -> bool {
    if Path::new("./config.cly").exists() {
        return false
    }

    let mut config: structs::Config = structs::Config {
        name: String::new(),
        version: None,
        platform: String::new(),
        file_extension: String::new(),
        hooks: Vec::new(),
        compiler: None,
        flags: None,
    };

    print!("{}", "Enter project name: ".cyan());
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut config.name) {
        Ok(_n) => {},
        Err(_e) => {
            println!("{}", "Failed to read line".red());
            return false
        }
    }
    config.name = config.name.trim().to_string();

    let config_json = to_string(&config).unwrap();
    let _ = fs::write("./config.cly", config_json);

    true
}