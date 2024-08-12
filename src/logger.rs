use clap::ArgMatches;
use log::LevelFilter;
use std::{env::consts, fs, path::Path, time::SystemTime};

pub fn setup_logger(matches: ArgMatches) -> Result<(), fern::InitError> {
    if !matches.get_flag("nologs") {
    let logdir =if consts::OS == "windows" {Path::new("C:\\Users\\%USERNAME%\\AppData\\Local\\Temp\\Catalyst")} else {Path::new("~/.catalyst/cache")};    
    if !Path::exists(logdir) {
        match fs::create_dir_all(logdir){
            Ok(_) => {}
            Err(_) => {
                println!("Failed to create logs directory");
            }
        }
    }

    
    let logfile = format!("{}/{}.log", logdir.display(), humantime::format_rfc3339_seconds(SystemTime::now()));
    match fs::File::create(logfile.clone()){
        Ok(_) => {}
        Err(_) => {
            println!("Failed to create logs file");
        }
    };
    let loglevel: LevelFilter;
    if matches.get_flag("debug") {
        loglevel = LevelFilter::Debug;
    }
    else if matches.get_flag("verbose") {
        loglevel = LevelFilter::Info;
    }
    else {
        loglevel = LevelFilter::Warn;
    }
    
    fern::Dispatch::new()
    .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level_for("globset", LevelFilter::Off)
        .level_for("fern", LevelFilter::Trace)
        .chain(fern::log_file(logfile.clone())?)
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
    }
    else {
        Ok(())
    }
}