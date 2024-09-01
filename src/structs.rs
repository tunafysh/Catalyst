use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: Option<String>,
    pub working_directory: String,
    pub hooks: Vec<String>
}