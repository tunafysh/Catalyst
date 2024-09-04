use std::{env, io::stdout, process::{exit, Command}};
use url::Url;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cly <url>");
        return;
    }

    let url = Url::parse(&args[1]).unwrap();
    if url.scheme() == "cly" {
        let path = &url.as_str()[6..];
        println!("Executing command: cly {}", path);
        let _ = Command::new("cly").arg(path).stdout(stdout()).spawn();
        exit(0);
    } else {
        eprintln!("Invalid URL: {}", url);
    }
}