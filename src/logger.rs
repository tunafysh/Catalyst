use clap::ArgMatches;
use log::LevelFilter;
use std::{env::consts, fs, path::Path, process::exit};
use fern::colors::{Color, ColoredLevelConfig};
use chrono;
use whoami::username;

pub fn setup_logger(matches: ArgMatches) -> Result<(), fern::InitError> {
    let logdir_linux_path = format!("/home/{}/.catalyst/logs", username());
    let logdir_windows_path = format!("C:\\Users\\{}\\AppData\\Local\\Temp\\Catalyst", username());

    let logdir = match consts::OS {
        "windows" => Path::new(logdir_windows_path.as_str()),
        "linux" => Path::new(logdir_linux_path.as_str()),
        _ => {
            println!("Unsupported OS: {}", consts::OS);
            exit(1);
        }
    };

        if !logdir.exists() {
            if let Err(err) = fs::create_dir_all(&logdir) {
                println!("Failed to create logs directory: {}", err);
                exit(1);
            }
        }

        let logfile_win = format!("{}\\catalyst-{}.log", logdir.display(), chrono::offset::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string());
        let logfile_linux = format!("{}/catalyst-{}.log", logdir.display(), chrono::offset::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string());

        let logfile = match consts::OS {
            "windows" => logfile_win,
            "linux" => logfile_linux,
            _ => {
                println!("Unsupported OS: {}", consts::OS);
                exit(1);
            }
        };

    match fs::File::create(logfile.clone()){
        Ok(_) => {}
        Err(e) => {
            println!("Failed to create logs file, error: {}", e);
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
        loglevel = LevelFilter::Off;
    }
    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .debug(Color::Cyan)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::Magenta);
    let stdout_config = {fern::Dispatch::new()
    .format(move |out, message, record| {
        out.finish(format_args!(
            "{color_line}[{date} {level} {color_line} {target} ] {message} {color_line}\x1B[0m",
            color_line = format_args!(
                "\x1B[{}m",
                colors.get_color(&record.level()).to_fg_str()
            ),
            date = chrono::offset::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string(),
            target = record.target(),
            level = colors.color(record.level()),
            message = message,
        ));
    })
        .level_for("reqwest", LevelFilter::Off)
        .level_for("globset", LevelFilter::Off)
        .level_for("fern", LevelFilter::Trace)
        .level(loglevel)
        .chain(std::io::stdout())
    };

    let file_config = {fern::Dispatch::new()
    .format(move |out, message, record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            chrono::offset::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string(),
            record.level(),
            record.target(),
            message,
        ));
    })
        .level_for("reqwest", LevelFilter::Off)
        .level_for("globset", LevelFilter::Off)
        .level_for("fern", LevelFilter::Trace)
        .chain(fern::log_file(logfile.clone().trim())?)
    };

    fern::Dispatch::new()
    .chain(stdout_config)
    .chain(file_config)
        .apply()?;

    Ok(())
}