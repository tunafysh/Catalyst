use colored::Colorize;
use std::{ process, fmt, env::args};
use sysinfo::System;

const EASEBUILD_VERSION: &str = "0.1";

fn main() {
    let args: Vec<String> = args().collect();
    if args.contains(&"--verbose".to_string()) || args.contains(&"-v".to_string()) {
        let sys = System::new_all();
        println!("{}", format!("Easebuild. Version {}, Platform: {}, Architecture: {}, Number of cores: {}, Memory: {} GB", EASEBUILD_VERSION.purple(), System::name().unwrap().purple(), System::cpu_arch().unwrap().to_string().purple(), sys.cpus().len().to_string().purple(), ((sys.total_memory() / 1024 / 1024 /1024) + 1).to_string().purple()).blue());
    }
    else {
        println!("{}", format!("Easebuild, Version {}", EASEBUILD_VERSION.purple()).blue());
    }
}
