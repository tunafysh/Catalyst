use std::{io::{stdin, stdout, Write}, process::exit};
use owo_colors::{OwoColorize, Stream::Stdout};
use log::{info, warn};

pub fn debug() {
    
    let _ = std::process::Command::new(if cfg!(target_family = "windows") {"cls"} else {"clear"}).status().unwrap().success();
    info!("Debug mode enabled, dropping to Debug shell");
    loop {
        let mut cmd = String::new();
        print!("{}", "Debug > ".if_supports_color(Stdout, |text| text.bright_purple()).bold());
        let _ = stdout().flush();
        stdin().read_line(&mut cmd).expect("Failed to read line");
        let cmd = cmd.trim();
        match cmd {
            "exit" => {
                warn!("Exiting");
                exit(0);
            }
            "clr" | "clear" => {
                let _ = std::process::Command::new(if cfg!(target_family = "windows") {"cls"} else {"clear"}).status().unwrap().success();
                continue;
            }
            "ping" => {
                info!("Pong!");
                continue;
            }
            "help" => {
                println!("{}", "Commands".if_supports_color(Stdout, |text| text.purple()).underline());
                println!("  exit");
                println!("  ping");
                println!("  clr, clear");
                continue;
            }
            _ => ()
        }
    }
}