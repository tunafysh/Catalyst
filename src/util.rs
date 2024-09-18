use std::{fs::{self, File}, io::{self, BufReader, BufWriter, Error, Read, Write}, path::Path, process::Command as Cmd, vec};
use anstyle::{AnsiColor, Color, Style};
use clap::{arg, builder::Styles, command, value_parser, ArgAction, ArgMatches, Command};
use hex_rgb::{convert_hexcode_to_rgb, Color as rgbcolor};
use hyperpolyglot::{get_language_breakdown, Language};
use log::{error, info};
use serde_json::to_string_pretty;
use sysinfo::System;
use walkdir::WalkDir;
use owo_colors::{OwoColorize, Stream};
use zip::{write::FullFileOptions, ZipArchive, ZipWriter};

use crate::{structs, updater, CATALYST_VERSION};

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

pub fn find_file<P: AsRef<Path>>(dir: P, file_name: Vec<&str>) -> Result<std::path::PathBuf, Error> {
    for entry in WalkDir::new(dir) {
        if let Ok(entry) = entry {
            for file in file_name.clone() {
                if entry.file_name().to_string_lossy().to_string().contains(file) {
                    return Ok(entry.path().to_path_buf())
                }
            }
        }
    }
    Err(Error::new(io::ErrorKind::NotFound, "File not found"))
}

pub fn generate() -> bool {
    if Path::new("./config.cly").exists() {
        return false
    }

    let mut config: structs::Config = structs::Config {
        name: String::new(),
        version: None,
        working_directory: String::new(),
        hooks: Vec::new()
    };

    let input = prompt("Enter project name: ".to_string());
    if let Some(input) = input {
        config.name = input;
    }
    else {
        println!("{}", "Project name cannot be empty.".if_supports_color(Stream::Stdout, |text| text.red()).if_supports_color(Stream::Stdout, |text| text.bold()));
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
        config.working_directory = input;
    }

        config.hooks = vec!["main".to_string()];

    config.name = config.name.trim().to_string();

    let config_json = to_string_pretty(&config).unwrap();
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

pub fn detect_languages() -> Vec<String> {
    
    let mut languages: Vec<String> = Vec::new();

    info!("Scanning current directory...");  

    let breakdown = get_language_breakdown("./");
    let mut total_files = 0;
    for (_language, detections) in &breakdown {
        total_files += detections.len();
    }

    let percentage = breakdown.iter().map(|(language, detections)| {
        languages.push(language.to_string());
        format!("{}: {}%", language, ((detections.len() as f64 / total_files as f64) * 100.0).round().to_string())
    }).collect::<Vec<_>>().join(", ");
    
    println!("{}", format!("Languages used:").to_string().if_supports_color(Stream::Stdout, |text| text.blue()));
        for lang in percentage.split(", ") {
            let language_struct = Language::try_from(lang.split(":").next().unwrap()).unwrap();
            let hex_color = language_struct.color;
            match hex_color {
                Some(hex_color) => {
                    let color: rgbcolor = convert_hexcode_to_rgb(hex_color.to_string()).unwrap();
                    print!("{}\n", format!("{}", lang.to_string()).to_string().if_supports_color(Stream::Stdout, |text| text.truecolor(color.red, color.green, color.blue)));
                }
                None => {
                    print!("{}\n", format!("{}", lang.to_string()).to_string().if_supports_color(Stream::Stdout, |text| text.truecolor(255, 255, 255)));
                }
            }
            
        }

    if languages.len() > 0 {
        let languages = languages.join(", ");
        println!("{}", format!("Total files: {}", total_files).to_string().if_supports_color(Stream::Stdout, |text| text.blue()));
        println!("{}", format!("Languages detected: {}", languages).to_string().if_supports_color(Stream::Stdout, |text| text.blue()));   
        println!("{}", format!("Done.").to_string().if_supports_color(Stream::Stdout, |text| text.blue()));
    }
    languages
}

pub fn shell(cmd: &str, stdout: bool) {

    let output = Cmd::new(cmd).output().unwrap();
    if stdout {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }     
}

pub fn args() -> ArgMatches {
    let styles = Styles::styled()
        .usage(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
        .header(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Yellow))))
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
        .error(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red))))
        .valid(Style::new().bold().underline().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Magenta))));

    let cmd = command!()
        .styles(styles)
        .version(CATALYST_VERSION)
        .arg(
            arg!(-c --config <FILE> "Configuration file to use, default: .catalyst/config.cly.json")
                .required(false)
                .action(ArgAction::Set)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(-n --nologs "Disables logging")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            arg!(-d --debug ... "Turn debug information on")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            arg!(-v --verbose ... "Turn verbose information on")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .subcommand(Command::new("init").about("Initializes a new configuration file"))
        .subcommand(Command::new("cleanup").about("Cleans up the logs."))
        .subcommand(Command::new("update").about("Updates the catalyst application."))
        .subcommand(Command::new("check").about("Checks for updates."));

    cmd.get_matches()
}

pub fn banner(matches: ArgMatches) {
    let availableupdates = updater::check(CATALYST_VERSION);
    if availableupdates {
        if matches.get_flag("verbose") {
            let sys = System::new_all();
            if matches.get_flag("debug") {
                println!("{}", format!("Catalyst. version {} {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB, {}", 
                CATALYST_VERSION.if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Update available".if_supports_color(Stream::Stdout, |text| text.bright_green()).if_supports_color(Stream::Stdout, |text| text.bold()),
                System::name().unwrap().if_supports_color(Stream::Stdout, |text| text.purple()), 
                System::cpu_arch().unwrap().to_string().if_supports_color(Stream::Stdout, |text| text.purple()),
                sys.cpus().len().to_string().if_supports_color(Stream::Stdout, |text| text.purple()), 
                ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Debug mode".if_supports_color(Stream::Stdout, |text| text.yellow()).if_supports_color(Stream::Stdout, |text| text.bold())).if_supports_color(Stream::Stdout, |text| text.blue()));
            }
            else {
                println!("{}", format!("Catalyst. version {} {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB", 
                CATALYST_VERSION.if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Update available".if_supports_color(Stream::Stdout, |text| text.bright_green()).if_supports_color(Stream::Stdout, |text| text.bold()),
                System::name().unwrap().if_supports_color(Stream::Stdout, |text| text.purple()), 
                System::cpu_arch().unwrap().to_string().if_supports_color(Stream::Stdout, |text| text.purple()),
                sys.cpus().len().to_string().if_supports_color(Stream::Stdout, |text| text.purple()), 
                ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().if_supports_color(Stream::Stdout, |text| text.purple())).if_supports_color(Stream::Stdout, |text| text.blue()));
            }
        }
    }
    else{
        if matches.get_flag("verbose") {
            let sys = System::new_all();
            if matches.get_flag("debug") {
                println!("{}", format!("Catalyst. version {} {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB, {}", 
                CATALYST_VERSION.if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Update available".if_supports_color(Stream::Stdout, |text| text.bright_green()).if_supports_color(Stream::Stdout, |text| text.bold()),
                System::name().unwrap().if_supports_color(Stream::Stdout, |text| text.purple()), 
                System::cpu_arch().unwrap().to_string().if_supports_color(Stream::Stdout, |text| text.purple()),
                sys.cpus().len().to_string().if_supports_color(Stream::Stdout, |text| text.purple()), 
                ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Debug mode".if_supports_color(Stream::Stdout, |text| text.yellow()).if_supports_color(Stream::Stdout, |text| text.bold())).if_supports_color(Stream::Stdout, |text| text.blue()));
            }
            else {
                println!("{}", format!("Catalyst. version {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB", 
                CATALYST_VERSION.purple(),
                System::name().unwrap().purple(), 
                System::cpu_arch().unwrap().to_string().purple(), 
                sys.cpus().len().to_string().purple(), 
                ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().purple()).blue());
            }
        }
        else {
            if matches.get_flag("debug") {
                println!("{}", format!("Catalyst, version {} {}, Platform: {}, {}",
                CATALYST_VERSION.if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Update available".if_supports_color(Stream::Stdout, |text| text.bright_green()).if_supports_color(Stream::Stdout, |text| text.bold()),
                System::name().unwrap().if_supports_color(Stream::Stdout, |text| text.purple()), 
                 "Debug mode".if_supports_color(Stream::Stdout, |text| text.yellow()).if_supports_color(Stream::Stdout, |text| text.bold())).if_supports_color(Stream::Stdout, |text| text.blue()));
            }
            else{
                println!("{}", format!("Catalyst, version {} {}, Platform: {}",
                CATALYST_VERSION.if_supports_color(Stream::Stdout, |text| text.purple()), 
                "Update available".if_supports_color(Stream::Stdout, |text| text.bright_green()).if_supports_color(Stream::Stdout, |text| text.bold()),
                System::name().unwrap().if_supports_color(Stream::Stdout, |text| text.purple())).if_supports_color(Stream::Stdout, |text| text.blue()));
            }
        }
    }
}

pub fn extract_zip(file: String, dest: String) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(file.as_str());
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = std::path::Path::new(dest.as_str());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

pub fn package_zip(file_paths: Vec<String>, zip_path: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(zip_path);
    let file = File::create(&path)?;
    let mut zip = ZipWriter::new(BufWriter::new(file));
    let options = FullFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for file_path in file_paths {
        let path = Path::new(file_path.as_str());
        let mut f = File::open(&path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        let _ = zip.start_file(
            path.file_name().unwrap().to_str().unwrap(),
            options.clone()
        )?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    Ok(())
}