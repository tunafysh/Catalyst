use std::{env::consts, fs, path::Path, process};
use owo_colors::{OwoColorize, Stream::Stdout};
use log::{error, info, warn};
use util::find_file;

mod structs;
mod util;
mod logger;
mod debug;
mod updater;
mod lua;

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

    if let Some(hooks) = matches.get_many::<String>("hook") {
        for hook in hooks {
            info!("{}", format!("Running hook: {}", hook).to_string().if_supports_color(Stdout, |text| text.cyan()));
            let file = find_file(".", vec![hook]).unwrap();
            let _ = lua::run_script(file.display().to_string());
        }
        process::exit(0);
    }

    let mut conf: structs::Config = structs::Config {
        name: String::new(),
        version: None,
        working_directory: String::new(),
        hooks: Vec::new(),
    };

    let config = matches.get_many::<String>("config").unwrap_or_default().into_iter().map(|v| v.as_str()).collect::<Vec<_>>();

        if config.len() != 0 {
            if !config[0].contains(".cly.json") {
                error!("{}", "Not a configuration file.".to_string());
                process::exit(2);
            }

            info!("{}", format!("Using configuration file: {}", config[0].if_supports_color(Stdout, |text| text.purple()).blue()));
        } else {
            info!("Scanning for config files...");
            match util::find_file(".catalyst/", vec!["config.cly.json"]) {
                Err(_) => {
                    error!("{}", "No config file found.".to_string());
                    println!("{}", "No config file found. Please create a configuration file as i don't know what this directory is...".to_string().bold().yellow());
                    process::exit(1);
                }
                Ok(path) => {
                    info!("Found config file: {}", path.display().to_string().if_supports_color(Stdout, |text| text.purple()));
                    let config = path.display().to_string();
                    info!("Parsing...");
                    conf = match fs::read_to_string(config) {
                        Err(_) => {
                            error!("{}", "Cannot read configuration file.".to_string());
                            process::exit(3);
                        }
                        Ok(cf) => match serde_json::from_str(&cf) {
                            Err(_) => {
                                error!("{}", "Inavlid configuration file.".to_string());
                                process::exit(4);
                            }
                            Ok(c) => c,
                        },
                    };
                }
            }
    }

    let confver = match conf.version {
        Some(v) => v,
        None => "None".to_string(),
    };

        println!(
            "{}",
            format!(
                "Configuration:\n\t+ Project name: {}, \n\t+ Project version: {}, \n\t+ Current working directory: {}, \n\t+ Hooks: {}",
                conf.name, confver, conf.working_directory, conf.hooks.join(", ")
            )
            .to_string()
            .if_supports_color(Stdout, |text| text.magenta())
        );

    if conf.hooks.len() != 0 {
        info!("Running hooks...");
        for hook in conf.hooks {
            info!("{}", format!(".catalyst/{}.cly", hook));
            println!("{}", format!("Running hook: {}", hook).to_string().if_supports_color(Stdout, |text| text.cyan()));
            let _ = lua::run_script(format!(".catalyst/{}.cly", hook));
        }
    } else {
        warn!("No hooks found. Please create a hook as i don't know what to do here...");
    }
}