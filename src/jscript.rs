use std::{env, fs, process::{self, Command}};
use quick_js::{console, Context, JsValue, Callback};
use git2::{Repository, SubmoduleUpdateOptions};
use log::{error, info, warn};

use reqwest::Client;

use crate::util::{extract_zip, find_file, package_zip, prompt, is_tool};

pub fn run_js(script: String) {
    let context = Context::builder()
        .console(console::LogConsole)
        .build()
        .unwrap();
    
        context.add_callback("info", |msg: String| -> JsValue {
            info!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("error", |msg: String| -> JsValue {
            error!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("warn", |msg: String| -> JsValue {
            warn!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("findfile", |path: String| -> JsValue {
            let path = find_file("src/.catalyst/", vec![path.as_str()]);
            let _ = match path {
                Ok(path) => Ok(path.display().to_string()),
                Err(_) => {
                    error!("File not found.");
                    Err(quick_js::ValueError::Internal("File not found".to_string()))
                }
            };
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("prompt", |msg: String| -> JsValue {
            match prompt(msg) {
                None => return JsValue::String(String::new()),
                Some(input) => {
                    return JsValue::String(input.clone().trim().to_string())
                }
            };
            
        }).unwrap();

        context.add_callback("warn", |msg: String| -> JsValue {
            warn!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("warn", |msg: String| -> JsValue {
            warn!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("warn", |msg: String| -> JsValue {
            warn!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

        context.add_callback("warn", |msg: String| -> JsValue {
            warn!("{}", msg);
            JsValue::Int(0)
        }).unwrap();

    let script = script.lines().skip(1).collect::<Vec<_>>().join("\n");

    context.eval(&script)
        .map_err(|e| error!("Failed to evaluate script: {}", e))
        .unwrap();
}