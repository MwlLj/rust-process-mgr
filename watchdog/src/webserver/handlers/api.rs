use crate::templates::html;
use crate::process::ProcessStatus;
use crate::process;
use crate::core::dispatch::CDispatch;
use super::{CDefaultResponse, CGetAllConfigResponse, CPutReloadRequest
, CGetOneProcessStatusResponse, CGetAllProcessStatusResponse, CStatus};
use crate::config::file;
use crate::config::{ConfigInfo, Process};

use tiny_http::{Request, Response, Header};
use serde_json;
use json::{JsonValue};

use std::collections::VecDeque;

const header_name: &str = "name";

pub struct CApiHandler {
}

impl CApiHandler {
    pub fn handleStopProcess(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let name = CApiHandler::findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            if let Err(err) = dispatch.stopProcess(&name) {
                res.result = false;
                res.status = *super::status_stop_process_error;
                break;
            };
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleStopProcessWithConfig(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let name = CApiHandler::findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            let ps = match dispatch.stopProcess(&name) {
                Ok(p) => p,
                Err(err) => {
                    res.result = false;
                    res.status = *super::status_stop_process_error;
                    break;
                }
            };
            let ps = match ps.lock() {
                Ok(p) => p,
                Err(err) => {
                    res.result = false;
                    res.status = *super::status_stop_process_error;
                    break;
                }
            };
            // save to file
            let fileOps = dispatch.fileOps();
            match fileOps.write(&ConfigInfo{
                processList: Some(ps.clone())
            }) {
                Ok(_) => {},
                Err(err) => {
                    println!("write to file error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_file_rw_error;
                    break;
                }
            }
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleStopProcessByAlias(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let alias = CApiHandler::findHeader(&request.headers(), header_name);
            let alias = match urlencoding::decode(&alias) {
                Ok(n) => n,
                Err(err) => {
                    res.result = false;
                    res.status = *super::status_param_error;
                    break;
                }
            };
            if alias == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            if let Err(err) = dispatch.stopProcessByAlias(&alias) {
                res.result = false;
                res.status = *super::status_stop_process_error;
                break;
            };
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleRestartProcess(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let name = CApiHandler::findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            if let Err(err) = dispatch.restartProcess(&name) {
                res.result = false;
                res.status = *super::status_restart_process_error;
                break;
            };
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleRestartProcessWithConfig(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let name = CApiHandler::findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            let ps = match dispatch.restartProcess(&name) {
                Ok(p) => p,
                Err(err) => {
                    res.result = false;
                    res.status = *super::status_restart_process_error;
                    break;
                }
            };
            let ps = match ps.lock() {
                Ok(p) => p,
                Err(err) => {
                    res.result = false;
                    res.status = *super::status_restart_process_error;
                    break;
                }
            };
            // save to file
            let fileOps = dispatch.fileOps();
            match fileOps.write(&ConfigInfo{
                processList: Some(ps.clone())
            }) {
                Ok(_) => {},
                Err(err) => {
                    println!("write to file error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_file_rw_error;
                    break;
                }
            }
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleRestartProcessByAlias(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let alias = CApiHandler::findHeader(&request.headers(), header_name);
            let alias = match urlencoding::decode(&alias) {
                Ok(n) => n,
                Err(err) => {
                    res.result = false;
                    res.status = *super::status_param_error;
                    break;
                }
            };
            if alias == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            if let Err(err) = dispatch.restartProcessByAlias(&alias) {
                res.result = false;
                res.status = *super::status_restart_process_error;
                break;
            };
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleStopAllProcess(dispatch: &mut CDispatch, request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            dispatch.stopAllProcess();
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleRestartAllProcess(dispatch: &mut CDispatch, request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            dispatch.restartAllProcess();
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleGetAllConfig(fileOps: &file::CFile, request: Request) {
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

    pub fn handleReload(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let mut reqStr = String::new();
            match request.as_reader().read_to_string(&mut reqStr) {
                Ok(_) => {
                },
                Err(err) => {
                    println!("read request body error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_body_read_error;
                    break;
                }
            }
            /*
            // let mut req: CPutReloadRequest = match serde_json::from_str(&reqStr) {
            let mut jv: JsonValue = match json::parse(&reqStr) {
                Ok(r) => r,
                Err(err) => {
                    println!("parse request json error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_json_parse_error;
                    break;
                }
            };
            let mut req = CPutReloadRequest::default();
            if jv["processList"].is_null() {
                req.processList = VecDeque::new();
            } else {
                req = match serde_json::from_str(&reqStr) {
                    Ok(r) => r,
                    Err(err) => {
                        println!("parse request json error, err: {}", err);
                        res.result = false;
                        res.status = *super::status_json_parse_error;
                        break;
                    }
                };
            }
            */
            let mut req: CPutReloadRequest = match serde_json::from_str(&reqStr) {
                Ok(r) => r,
                Err(err) => {
                    println!("parse request json error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_json_parse_error;
                    break;
                }
            };
            let mut processList = match req.processList {
                Some(pl) => pl,
                None => {
                    VecDeque::new()
                }
            };
            dispatch.reload(&mut processList);
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleSaveBeforeReload(dispatch: &mut CDispatch, mut request: Request) {
        let mut res = CDefaultResponse::default();
        loop {
            let mut reqStr = String::new();
            match request.as_reader().read_to_string(&mut reqStr) {
                Ok(_) => {
                },
                Err(err) => {
                    println!("read request body error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_body_read_error;
                    break;
                }
            }
            let mut req: CPutReloadRequest = match serde_json::from_str(&reqStr) {
                Ok(r) => r,
                Err(err) => {
                    println!("parse request json error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_json_parse_error;
                    break;
                }
            };
            let mut processList = match req.processList {
                Some(pl) => pl,
                None => {
                    VecDeque::new()
                }
            };
            /*
            let mut jv: JsonValue = match json::parse(&reqStr) {
                Ok(r) => r,
                Err(err) => {
                    println!("parse request json error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_json_parse_error;
                    break;
                }
            };
            let mut req = CPutReloadRequest::default();
            if jv["processList"].is_null() {
                req.processList = VecDeque::new();
            } else {
                req = match serde_json::from_str(&reqStr) {
                    Ok(r) => r,
                    Err(err) => {
                        println!("parse request json error, err: {}", err);
                        res.result = false;
                        res.status = *super::status_json_parse_error;
                        break;
                    }
                };
            }
            */
            // save to file
            let fileOps = dispatch.fileOps();
            match fileOps.write(&ConfigInfo{
                processList: Some(processList.clone())
            }) {
                Ok(_) => {},
                Err(err) => {
                    println!("write to file error, err: {}", err);
                    res.result = false;
                    res.status = *super::status_file_rw_error;
                    break;
                }
            }
            // dispatch.reload(&mut req.processList);
            dispatch.reload(&mut processList);
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleGetOneStatusRequest(dispatch: &CDispatch, request: Request) {
        let mut res = CGetOneProcessStatusResponse::default();
        loop {
            let name = CApiHandler::findHeader(&request.headers(), header_name);
            if name == "" {
                res.result = false;
                res.status = *super::status_param_error;
                break;
            }
            let status = match dispatch.getRunStatus(&name, "") {
                Ok(s) => s,
                Err(err) => {
                    println!("get runStatus error, err: {}", err);
                    break;
                }
            };
            res.data = CStatus {
                pid: status.pid,
                runTime: status.runTime,
                status: process::to_status_desc(&status.status),
                name: status.name,
                alias: status.alias
            };
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }

    pub fn handleGetAllStatusRequest(dispatch: &CDispatch, request: Request) {
        let mut res = CGetAllProcessStatusResponse::default();
        loop {
            let statuses = match dispatch.getAllRunStatus() {
                Ok(s) => s,
                Err(err) => {
                    println!("get all runStatus error, err: {}", err);
                    break;
                }
            };
            for status in statuses {
                res.data.push(CStatus{
                    pid: status.pid,
                    runTime: status.runTime,
                    status: process::to_status_desc(&status.status),
                    name: status.name,
                    alias: status.alias
                });
            }
            break;
        }
        res.message = super::to_message(&res.status);
        request.respond(Response::from_data(serde_json::to_string(&res).unwrap().as_bytes()));
    }
}

impl CApiHandler {
    fn findHeader(headers: &[Header], key: &'static str) -> String {
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
