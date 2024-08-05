use clap::ArgMatches;
use log::LevelFilter;
use std::{time::SystemTime, fs};

pub fn setup_logger(matches: ArgMatches) -> Result<(), fern::InitError> {
    fs::create_dir("logs")?;
    fs::File::create("logs/output.log")?;
    if matches.get_flag("debug") {
        log::set_max_level(LevelFilter::Debug);
    }
    else if matches.get_flag("verbose") {
        log::set_max_level(LevelFilter::Trace);
    }
    else {
        log::set_max_level(LevelFilter::Off);
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
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("logs/output.log")?)
        .apply()?;
    Ok(())
}