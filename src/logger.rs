use clap::ArgMatches;
use log::{error, LevelFilter};
use std::{env::consts, fs, path::Path, process::exit, time::SystemTime};
use fern::colors::{Color, ColoredLevelConfig};
use whoami::username;

pub fn setup_logger(matches: ArgMatches) -> Result<(), fern::InitError> {
    let username = username();
    if !matches.get_flag("nologs") {
        let logdir_path = format!("/home/{}/.catalyst/logs", username);
let logdir = if consts::OS == "windows" {
    Path::new("C:\\Users\\%USERNAME%\\AppData\\Local\\Temp\\Catalyst")
} else {
    Path::new(logdir_path.as_str())
};
    if !Path::exists(logdir) {
        match fs::create_dir_all(logdir){
            Ok(_) => {}
            Err(_) => {
                error!("Failed to create logs directory");
                exit(1);
            }
        }
    }

    
    let logfile = format!("{}/{}.log", logdir.display(), humantime::format_rfc3339_seconds(SystemTime::now()));
    match fs::File::create(logfile.clone()){
        Ok(_) => {}
        Err(_) => {
            error!("Failed to create logs file");
            exit(1);
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
    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .debug(Color::Cyan)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::Magenta);
    fern::Dispatch::new()
    .format(move |out, message, record| {
        out.finish(format_args!(
            "{color_line}[{date} {level} {color_line} {target} ] {message} {color_line}\x1B[0m",
            color_line = format_args!(
                "\x1B[{}m",
                colors.get_color(&record.level()).to_fg_str()
            ),
            date = humantime::format_rfc3339_seconds(SystemTime::now()),
            target = record.target(),
            level = colors.color(record.level()),
            message = message,
        ));
    })
        .level_for("globset", LevelFilter::Off)
        .level_for("fern", LevelFilter::Trace)
        .chain(fern::log_file(logfile.clone().trim())?)
        .level(loglevel)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
    }
    else {
        Ok(())
    }
}