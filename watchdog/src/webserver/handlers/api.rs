use crate::templates::html;
use crate::process::ProcessStatus;
use crate::core::dispatch::CDispatch;
use super::{CDefaultResponse, CGetAllConfigResponse};
use crate::config::file;

use tiny_http::{Request, Response, Header};
use serde_json;

const header_name: &str = "name";

pub struct CApiHandler {
}

impl CApiHandler {
    pub fn handleStopProcess(&self, dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let name = self.findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            dispatch.stopProcess(&name);
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleRestartProcess(&self, dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let name = self.findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            dispatch.restartProcess(&name);
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleStopAllProcess(&self, dispatch: &mut CDispatch, request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            dispatch.stopAllProcess();
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleRestartAllProcess(&self, dispatch: &mut CDispatch, request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            dispatch.restartAllProcess();
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleGetAllConfig(&self, fileOps: &file::CFile, request: Request) {
        let mut res = CGetAllConfigResponse::default();
        loop {
            match fileOps.read() {
                Ok((config, _)) => {
                    res.data = config.processList;
                },
                Err(err) => {
                    break;
                }
            }
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }
}

impl CApiHandler {
    fn findHeader(&self, headers: &[Header], key: &'static str) -> String {
        let mut value = String::new();
        for item in headers {
            if item.field.equiv(key) {
                value = item.value.as_str().to_string();
                break;
            }
        }
        value
    }
}

impl CApiHandler {
    pub fn new() -> CApiHandler {
        CApiHandler{
        }
    }
}
