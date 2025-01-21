use std::{io::{stdin, stdout, Write}, process::exit};
use owo_colors::{OwoColorize, Stream::Stdout};
use clearscreen::clear;
use log::{info, warn, error, debug};

use crate::{lua::run_lua, util::{banner, args, find_file}};

pub fn debug() {
    clear().expect("failed to clear screen");
    banner(args());
    info!("Debug mode enabled, dropping to Debug shell");
    loop {
        let mut cmd = String::new();
        print!("{}", "\nDebug > ".if_supports_color(Stdout, |text| text.bright_purple()).bold());
        let _ = stdout().flush();
        stdin().read_line(&mut cmd).expect("Failed to read line");
        let words: Vec<&str> = cmd.split_whitespace().collect();
        print!("\n");
        match words[0] {
            "exit" => {
                warn!("Exiting");
                exit(0);
            }
            "clr" | "clear" => {
                clear().expect("failed to clear screen");
                continue;
            }
            "ping" => {
                info!("Pong!");
                warn!("Pong!");
                error!("Pong!");
                debug!("Pong!");
                println!("Pong!");
                continue;
            }
            "info" => {
                if words.len() == 1 {
                    println!("{}", "Missing argument".if_supports_color(Stdout, |text| text.red()).bold());
                }
                else {
                    info!("{}", words[1]);
                }
            }
            "warn" => {
                if words.len() == 1 {
                    println!("{}", "Missing argument".if_supports_color(Stdout, |text| text.red()).bold());
                }
                else {
                    warn!("{}", words[1]);
                }
            }
            "debug" => {
                if words.len() == 1 {
                    println!("{}", "Missing argument".if_supports_color(Stdout, |text| text.red()).bold());
                }
                else {
                    debug!("{}", words[1]);
                }
            }
            "error" => {
                if words.len() == 1 {
                    println!("{}", "Missing argument".if_supports_color(Stdout, |text| text.red()).bold());
                }
                else {
                    error!("{}", words[1]);
                }
            }
            "hook" => {
                if words.len() == 1 {
                    println!("{}", "Missing argument".if_supports_color(Stdout, |text| text.red()).bold());
                }
                else {
                    let path = find_file(".", vec![words[1]]).unwrap();
                    run_lua(path.display().to_string()).unwrap();
                }
            }
            "help" => {
                println!("{}", "Commands".if_supports_color(Stdout, |text| text.purple()).underline());
                println!("  exit");
                println!("  ping");
                println!("  clr, clear");
                println!("  info, warn, error, debug <TEXT>");
                println!("  hook <HOOK-PATH>");
                continue;
            }
            _ => println!("{}", "Unknown command".if_supports_color(Stdout, |text| text.red()).bold())
        }
    }
}