use std::{env::consts, fs, path::Path, process};
use colored::Colorize;
use clap::{arg, builder::Styles, command, value_parser, ArgAction, ArgMatches, Command};
use anstyle::{Style, Color, AnsiColor};
use log::{error, info};
use sysinfo::System;    

mod structs;
mod util;
mod logger;
mod lua;

const CATALYST_VERSION: &str = "0.54";

fn args() -> ArgMatches {
    let styles = Styles::styled()
    .usage(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
    .header(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
        .error(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
        .valid(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Magenta))));
    
    let cmd = command!() // requires `cargo` feature
        .styles(styles)
        .version(CATALYST_VERSION)
        .arg(
            arg!(
                -c --config <FILE> "Configuration file to use, default: .catalyst/config.cly.json"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                -n --nologs "Disables logging"
            )
            .action(ArgAction::SetTrue)
            .required(false)
        )
        .arg(
            arg!(
                -d --debug ... "Turn debug information on"
            )
            .action(ArgAction::SetTrue)
            .required(false),
        )
        .arg(arg!(
            -v --verbose ... "Turn verbose information on"
        )
        .action(ArgAction::SetTrue)
        .required(false)
    )
    .subcommand(
        Command::new("init").about("Initializes a new configuration file")
    )
    .subcommand(
        Command::new("cleanup").about("Cleans up the logs.")
    )

    .get_matches();

    cmd
}

fn main() {
    let matches = args();
    if matches.get_flag("nologs") {
        
        match logger::setup_logger(matches.clone()) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", format!("Failed to setup logger. Error: {}", err).to_string().red());
                process::exit(1);
            },
        }
    }

    if matches.get_flag("verbose") {
        let sys = System::new_all();
        if matches.get_flag("debug") {
            println!("{}", format!("Catalyst. version {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB, {}", CATALYST_VERSION.purple(), System::name().unwrap().purple(), System::cpu_arch().unwrap().to_string().purple(), sys.cpus().len().to_string().purple(), ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().purple(), "Debug mode".yellow().bold()).blue());
        }
        else {
            println!("{}", format!("Catalyst. version {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB", CATALYST_VERSION.purple(), System::name().unwrap().purple(), System::cpu_arch().unwrap().to_string().purple(), sys.cpus().len().to_string().purple(), ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().purple()).blue());
        }
    }
    else {
        if matches.get_flag("debug") {
            println!("{}", format!("Catalyst, version {}, Platform: {}, {}", CATALYST_VERSION.purple(), System::name().unwrap().purple(), "Debug mode".yellow().bold()).blue());
        }
        else{
            println!("{}", format!("Catalyst, version {}, Platform: {}", CATALYST_VERSION.purple(), System::name().unwrap().purple()).blue());
        }
    }
    
    match matches.subcommand() {
        Some(("init", _)) => {
            info!("Initializing configuration...");
            let gensuccess = util::generate();
            process::exit(gensuccess as i32);
        }
        _ => {}
    }

    match matches.subcommand() {
        Some(("cleanup", _)) => {
            let logdir =if consts::OS == "windows" {Path::new("C:\\Users\\%USERNAME%\\AppData\\Local\\Temp\\Catalyst")} else {Path::new("~/.catalyst/cache")};    
            if !Path::exists(logdir) {
                match fs::create_dir(logdir){
                    Ok(_) => {}
                    Err(_) => {
                        error!("Failed to create logs directory");
                    }
                }
            }
            else {
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

    let mut conf: structs::Config = structs::Config {
        name: String::new(),
        version: None,
        working_directory: None,
        hooks: Vec::new(),
        compiler: None,
        flags: vec!["".to_string()],
    };

    let config = matches.get_many::<String>("config").unwrap_or_default().into_iter().map(|v| v.as_str()).collect::<Vec<_>>();
    if matches.get_flag("debug") == false {

        if config.len() != 0 {
            if !config[0].contains(".cly.json") {
                println!("{}", "Not a configuration file.".to_string().red());
            process::exit(2);
        }

        info!("{}", format!("Using configuration file: {}", config[0].purple()).blue());
    }
    else {
        info!("Scanning for config files...");
        if let Some(path) = util::find_file("src/.catalyst/", vec!["config.cly.json"]) {
            info!("Found config file: {}", path.display().to_string().purple());
            let config = path.display().to_string();
            info!("Parsing...");
            let configfile = match fs::read_to_string(config) {
                Err(_e) => {
                    error!("{}", "Cannot read configuration file.".to_string().red());
                    process::exit(3);
                }
                Ok(cf) => cf
            };
            
            conf = match serde_json::from_str(configfile.as_str()) {
                Err(_e) => {
                    error!("{}", "Inavlid configuration file.".to_string().red());
                    process::exit(4);
                }
                Ok(c) => c
            } 
        }
        else {
            error!("{}", "No config file found.".to_string().red());
            process::exit(1);
        }
        
    }    
}
    
    if !matches.get_flag("debug") {
        println!("{}", format!("Configuration:\n\t+ Project name: {}", conf.name.to_string()).to_string().magenta());
    }

    if conf.hooks.len() != 0 {
        info!("Running hooks...");
        for hook in conf.hooks {
            info!("{}", format!("src/.catalyst/{}.cly", hook));
            println!("{}", format!("Running hook: {}", hook).to_string().cyan());
            let _ = lua::run_script(format!("src/.catalyst/{}.cly", hook));
        }
    }
    else { 
        info!("No hooks found. Compiling all files in current project");
        util::compile_all();
    }
}