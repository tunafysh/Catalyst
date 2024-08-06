use clap::ArgMatches;
use log::LevelFilter;
use std::{fs, path::Path, time::SystemTime};

pub fn setup_logger(matches: ArgMatches) -> Result<(), fern::InitError> {
    if Path::exists(Path::new(".catalyst/logs")) {
        match fs::remove_dir_all(".catalyst/logs")
        {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to remove logs directory");
            }
        }
    }

        match fs::create_dir(".catalyst/logs"){
            Ok(_) => {}
            Err(_) => {
                println!("Failed to create logs directory");
            }
        }
    
    
    match fs::File::create(".catalyst/logs/output.log"){
        Ok(_) => {}
        Err(_) => {
            println!("Failed to create logs file");
        }
    }
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
        .chain(fern::log_file(".catalyst/logs/output.log")?)
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}