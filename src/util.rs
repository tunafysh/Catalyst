use std::{fs, path::Path};
use clap::ArgMatches;
use colored::Colorize;

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