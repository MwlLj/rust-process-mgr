use std::collections::VecDeque;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Process {
    pub name: String,
    pub alias: String,
    pub execute: String,
    pub args: Vec<String>,
    pub directory: String,
    pub isAuto: bool,
    pub restartTimeS: i64
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigInfo {
    pub processList: VecDeque<Process>
}

pub mod file;

