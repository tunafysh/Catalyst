use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: Option<String>,
    pub platform: String,
    pub file_extension: String,
    pub hooks: Vec<String>,
    pub compiler: Option<String>,
    pub flags: Option<Vec<String>>,
}