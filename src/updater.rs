use std::process::exit;
use owo_colors::{OwoColorize, Stream};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use sysinfo::{self, System};
use log::{info, error};

use crate::util::shell;

pub fn update(ver: &str) {
    info!("Initiating update mechanism");
    let client = Client::new();
    let mut osname = "Unix".to_string();
    if System::name().unwrap() == "windows" {
        osname = "Windows".to_string();
    }
    
    let user_agent = format!("Catalyst/{}/{}/{}", osname, ver, "update");

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent.parse().unwrap());

    info!("User agent built, sending request");

    let res = client.post("https://cly-rs.vercel.app/")
        .headers(headers)
        .send()
        .unwrap();

    let response = res.text().unwrap();

    if response == "Equalver" {
        println!("{}", "Catalyst is already up to date.".blue());
    } else if response == "Largerver" {
        println!("{}", "Cannot update because the this build may have been tampered or it is a custom build. Please reinstall...".red());
        error!("Tried to update but app was modified.");
    }
    else {
        info!("Updating...");
        if System::name().unwrap() == "windows" {
            shell(format!("powershell -c {}", response).as_str(), false);
        }
        else {
            shell(format!("bash -c {}", response).as_str(), false);
        }

        info!("Finished updating.");
        exit(0);
    }
}

pub fn check(ver: &str) -> bool {
    info!("Checking for updates");
    let client = Client::new();
    let mut osname = "Unix".to_string();
    if System::name().unwrap() == "windows" {
        osname = "Windows".to_string();
    }
    
    let user_agent = format!("Catalyst/{}/{}/{}", osname, ver, "check");

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent.parse().unwrap());

    info!("User agent built, sending request");

    let res = client.post("https://cly-rs.vercel.app/")
        .headers(headers)
        .send()
        .unwrap();

    let response = res.text().unwrap();

    if response == "Equalver" {
        println!("{}", "Catalyst is already up to date.".if_supports_color(Stream::Stdout, |text| text.blue()));
        return true;
    } else if response == "largerver" {
        println!("{}", "Cannot update because the this build may have been tampered or it is a custom build. Please reinstall...".if_supports_color(Stream::Stdout, |text| text.red()));
        error!("Tried to update but app was modified.");
    }
    else if response == "updateavailable" {
        return true;
    }

    return false;
}