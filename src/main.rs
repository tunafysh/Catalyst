use std::{ process, fs };
use colored::Colorize;
use clap::{arg, builder::Styles, command, value_parser, ArgAction, ArgMatches};
use anstyle::{Style, Color, AnsiColor};
use sysinfo::System;

mod structs;
mod util;

const EASEBUILD_VERSION: &str = "0.1";

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
        .version(EASEBUILD_VERSION)
        .arg(
            arg!(
                -c --config <FILE> "Configuration file to use, default: ./config.esb"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(String)),
        )
        .arg(arg!(
            -v --verbose ... "Turn verbose information on"
        )
        .action(ArgAction::SetTrue)
        .required(false)
    )
        .get_matches();

    cmd
}

fn main() {
    
    let mut conf: structs::Config = structs::Config {
        name: String::new(),
        version: None,
        platform: String::new(),
        arch: String::new(),
        file_extension: String::new(),
        compiler: None,
        flags: None,
    };

    let matches = args();
    
    if matches.get_flag("verbose") {
        let sys = System::new_all();
        println!("{}", format!("Easebuild. version {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB", EASEBUILD_VERSION.purple(), System::name().unwrap().purple(), System::cpu_arch().unwrap().to_string().purple(), sys.cpus().len().to_string().purple(), ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().purple()).blue());
    }
    else {
        println!("{}", format!("Easebuild, version {}, Platform: {}", EASEBUILD_VERSION.purple(), System::name().unwrap().purple()).blue());
    }

    let config = matches.get_many::<String>("config").unwrap_or_default().into_iter().map(|v| v.as_str()).collect::<Vec<_>>();
    if config.len() != 0 {
        util::verbose(matches.clone(), format!("{}", format!("Using configuration file: {}", config[0].purple()).blue()));
    }
    else {
        util::verbose(matches.clone(), "Scanning for config files...".to_string());
        if let Some(path) = util::find_file("./", "config.esb") {
            util::verbose(matches.clone(), format!("Found config file: {}", path.display().to_string().purple()));
            let config = path.display().to_string();
            util::verbose(matches.clone(), "Parsing...".to_string());
            let configfile = match fs::read_to_string(config) {
                Err(_e) => {
                    println!("{}", "Cannot read configuration file.".to_string().red());
                    process::exit(2);
                }
                Ok(cf) => cf
            };

            conf = match serde_json::from_str(configfile.as_str()) {
                Err(_e) => {
                    println!("{}", "Inavlid configuration file.".to_string().red());
                    process::exit(3);
                }
                Ok(c) => c
            } 
        }
        else {
            print!("{}", "No config file found.".to_string().red());
            process::exit(1);
        }
        
    }    

    print!("{}", format!("Configuration:\n\t+ Project name: {}", conf.name.to_string()).to_string().magenta());
}