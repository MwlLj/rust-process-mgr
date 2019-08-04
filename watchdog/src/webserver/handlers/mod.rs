use crate::config::Process;

use serde_derive::{Deserialize, Serialize};

use std::collections::VecDeque;

pub const status_ok: &i32 = &0;
pub const status_param_error: &i32 = &1;
pub const status_json_parse_error: &i32 = &2;
pub const status_body_read_error: &i32 = &3;
pub const status_file_rw_error: &i32 = &4;

pub fn to_message(status: &i32) -> String {
    if status == status_ok {
        return String::from("ok");
    } else if status == status_param_error {
        return String::from("param error");
    } else if status == status_json_parse_error {
        return String::from("json parse error");
    } else if status == status_body_read_error {
        return String::from("body read error");
    } else if status == status_file_rw_error {
        return String::from("file rw error");
    }
    "".to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CDefaultResponse {
    pub result: bool,
    pub status: i32,
    pub message: String
}

impl std::default::Default for CDefaultResponse {
    fn default() -> Self {
        CDefaultResponse {
            result: true,
            status: *status_ok,
            message: to_message(status_ok)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CGetAllConfigResponse {
    pub data: VecDeque<Process>,
    pub result: bool,
    pub status: i32,
    pub message: String
}

impl std::default::Default for CGetAllConfigResponse {
    fn default() -> Self {
        CGetAllConfigResponse {
            data: Default::default(),
            result: true,
            status: *status_ok,
            message: to_message(status_ok)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CPutReloadRequest {
    pub processList: VecDeque<Process>,
    pub result: bool,
    pub status: i32,
    pub message: String
}

pub mod auth;
pub mod index;
pub mod web;
pub mod api;
