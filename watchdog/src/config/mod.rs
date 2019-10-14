use std::collections::VecDeque;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Process {
    pub name: String,
    pub alias: Option<String>,
    pub execute: String,
    pub args: Vec<String>,
    pub directory: String,
    pub isAuto: bool,
    pub restartTimeS: Option<i64>
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigInfo {
    pub processList: Option<VecDeque<Process>>
}

pub mod file;

