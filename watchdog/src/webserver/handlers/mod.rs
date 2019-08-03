use crate::config::Process;

use serde_derive::{Deserialize, Serialize};

use std::collections::VecDeque;

pub const status_ok: &i32 = &0;
pub const status_param_error: &i32 = &1;

pub fn to_message(status: &i32) -> String {
    if status == status_ok {
        return String::from("ok");
    } else if status == status_param_error {
        return String::from("param error");
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

pub mod auth;
pub mod index;
pub mod web;
pub mod api;
