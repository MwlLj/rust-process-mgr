extern crate serde_json;
extern crate serde;
extern crate serde_derive;

use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::prelude::*;
use std::default::Default;

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Process {
    pub name: String,
    pub args: Vec<String>,
    pub directory: String,
    pub isAuto: bool
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigInfo {
    pub process_list: Vec<Process>,
}

pub struct CConfig {
}

impl CConfig {
    pub fn read(&self, path: &str) -> ConfigInfo {
        let processes: Vec<Process> = Vec::new();
        let mut configInfo = ConfigInfo{
            process_list: processes
        };
        if !Path::new(path).exists() {
            println!("not exists, {}", path);
            if let Ok(f) = File::create(path) {
                let mut writer = BufWriter::new(f);
                let data = r#"{
    "process_list": [
        {
            "name": "log_server",
            "args": [
                "-port", "50005"
            ],
            "directory": ".",
            "isAuto": true
        }
    ]
}"#;
                writer.write(data.as_bytes()).unwrap();
                writer.flush().unwrap();
                configInfo = serde_json::from_str(data).unwrap();
            };
        } else {
            if let Ok(f) = File::open(path) {
                let mut reader = BufReader::new(f);
                let mut buf = String::new();
                reader.read_to_string(&mut buf).unwrap();
                configInfo = serde_json::from_str(&buf).unwrap();
            };
        }
        configInfo
    }

    pub fn new() -> CConfig {
        let config = CConfig{};
        config
    }
}

