use std::{fs, io::{self, Write}, path::Path};
use hex_rgb::{convert_hexcode_to_rgb, Color};
use hyperpolyglot::{get_language_breakdown, Language};
use serde_json::to_string;
use walkdir::WalkDir;
use clap::ArgMatches;
use colored::Colorize;

use crate::structs;

pub fn prompt(msg: String) -> Option<String> {
    print!("{}", msg);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    if input.is_empty() {
        None
    }
    else {
        Some(input.trim().to_string())
    }
}

pub fn find_file<P: AsRef<Path>>(dir: P, file_name: Vec<&str>) -> Option<std::path::PathBuf> {
    for entry in WalkDir::new(dir) {
        if let Ok(entry) = entry {
            for file in file_name.clone() {
                if entry.file_name().to_string_lossy().to_string().contains(file) {
                    return Some(entry.path().to_path_buf())
                }
            }
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
        working_directory: None,
        hooks: Vec::new(),
        compiler: None,
        flags: Vec::new(),
    };

    let input = prompt("Enter project name: ".to_string());
    if let Some(input) = input {
        config.name = input;
    }
    else {
        println!("{}", "Project name cannot be empty.".red());
        return false
    }

    config.name = config.name.trim().to_string();

    let _input = prompt("Enter project version (can be left blank): ".to_string());
    if let Some(input) = _input {
        config.version = Some(input);
    }
    else {
        config.version = None;
    }

    let _input = prompt("Enter working directory (can be left blank): ".to_string());
    if let Some(input) = _input {
        config.working_directory = Some(input);
    }
    else {
        config.working_directory = None;
    }

    let _input = prompt("Enter compiler (can be left blank): ".to_string());
    if let Some(input) = _input {
        config.compiler = Some(input);
    }
    else {
        config.compiler = None;
    }

    let _input = prompt("Enter flags (can be left blank): ".to_string());
    if let Some(input) = _input {
        config.flags = input.split(" ").map(|s| s.to_string()).collect();
    }
    else {
        config.flags = vec!["".to_string()];
    }

    let _input = prompt("Enter hooks (can be left blank): ".to_string());
    if let Some(input) = _input {
        config.hooks = input.split(" ").map(|s| s.to_string()).collect();
    }
    else {
        config.hooks = Vec::new();
    }
    config.name = config.name.trim().to_string();

    let config_json = to_string(&config).unwrap();
    fs::DirBuilder::new().create(".catalyst").unwrap();
    let _ = fs::write(".catalyst/config.cly.json", config_json);

    true
}

pub fn get_compiler(lang:&str) -> Vec<&str> {
    match lang {
        "C" | "Cpp" => vec!["gcc", "clang"],
        "CSharp" => vec!["dotnet", "mcs"],
        "Go" => vec!["go"],
        "Haskell" => vec!["ghc"],
        "Java" => vec!["javac"],
        "Kotlin" => vec!["kotlinc"],
        "Python" => vec!["python"],
        "Ruby"  => vec!["ruby"],
        "Rust"  => vec!["rustc"],
        "Swift" => vec!["swiftc"],
        "Assembly" => vec!["nasm"],
        &_ => vec!["Unknown compiler, please specify it in config.cly"],
    }
}

pub fn compile_all(matches: ArgMatches) {
    
    let mut languages: Vec<String> = Vec::new();

    verbose(matches.clone(), "Scanning current directory...".to_string());  

    let breakdown = get_language_breakdown("./");
    let mut total_files = 0;
    for (_language, detections) in &breakdown {
        total_files += detections.len();
    }

    let percentage = breakdown.iter().map(|(language, detections)| {
        languages.push(language.to_string());
        format!("{}: {}%", language, ((detections.len() as f64 / total_files as f64) * 100.0).round().to_string())
    }).collect::<Vec<_>>().join(", ");
    
    println!("{}", format!("Languages used:").to_string().blue());
        for lang in percentage.split(", ") {
            let language_struct = Language::try_from(lang.split(":").next().unwrap()).unwrap();
            let hex_color = language_struct.color;
            match hex_color {
                Some(hex_color) => {
                    let color: Color = convert_hexcode_to_rgb(hex_color.to_string()).unwrap();
                    print!("{}\n", format!("{}", lang.to_string()).to_string().truecolor(color.red, color.green, color.blue));
                }
                None => {
                    print!("{}\n", format!("{}", lang.to_string()).to_string().truecolor(255, 255, 255));
                }
            }
            
        }
    
}