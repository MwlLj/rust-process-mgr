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
use std::collections::VecDeque;

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use super::{ConfigInfo, Process};

pub struct CFile {
    path: String
}

impl CFile {
    pub fn read(&self) -> Result<(ConfigInfo, String), &str> {
        let processes: VecDeque<Process> = VecDeque::new();
        let mut configInfo = ConfigInfo{
            processList: processes
        };
        let mut buf = String::new();
        if !Path::new(&self.path).exists() {
            println!("not exists, {}", &self.path);
            if let Ok(f) = File::create(&self.path) {
                let mut writer = BufWriter::new(f);
                let data = r#"{
    "processList": [
        {
            "name": "test1",
            "alias": "test1",
            "execute": "test",
            "args": [
                "-port", "50005"
            ],
            "directory": ".",
            "isAuto": true,
            "restartTimeS": 0
        }
    ]
}"#;
                if let Err(err) = writer.write(data.as_bytes()) {
                    println!("write error, err: {}", err);
                    return Err("write error");
                };
                if let Err(err) = writer.flush() {
                    println!("flush error, err: {}", err);
                    return Err("flush error");
                };
                configInfo = match serde_json::from_str(data) {
                    Ok(c) => c,
                    Err(err) => {
                        println!("json parse error, err: {}", err);
                        return Err("json parse error");
                    }
                };
            };
        } else {
            if let Ok(f) = File::open(&self.path) {
                let mut reader = BufReader::new(f);
                if let Err(err) = reader.read_to_string(&mut buf) {
                    println!("read to string error, err: {}", err);
                    return Err("read to string error");
                };
                configInfo = match serde_json::from_str(&buf) {
                    Ok(c) => c,
                    Err(err) => {
                        println!("json parse error, err: {}", err);
                        return Err("json parse error");
                    }
                };
            };
        }
        Ok((configInfo, buf))
    }

    pub fn write(&self, info: &ConfigInfo) -> Result<(), &str> {
        let s = match serde_json::to_string_pretty(info) {
            Ok(s) => s,
            Err(err) => {
                println!("write file error, json to_string error, err: {}", err);
                return Err("json to_string error");
            }
        };
        let file = match OpenOptions::new().write(true).create(true).truncate(true).open(&self.path) {
            Ok(f) => f,
            Err(err) => {
                println!("file open error, err: {}", err);
                return Err("file open error");
            }
        };
        let mut writer = BufWriter::new(file);
        if let Err(err) = writer.write(s.as_bytes()) {
            println!("write error, err: {}", err);
            return Err("write error");
        };
        if let Err(err) = writer.flush() {
            println!("flush error, err: {}", err);
            return Err("flush error");
        };
        Ok(())
    }

    pub fn new(path: &str) -> CFile {
        let file = CFile{
            path: path.to_string()
        };
        file
    }
}

