use crate::templates::html;
use crate::process::ProcessStatus;
use crate::core::dispatch::CDispatch;

use tiny_http::{Request, Response};

pub struct CWebHandler {
}

impl CWebHandler {
    pub fn handleStopProcess(dispatch: &mut CDispatch, mut request: Request) {
        loop {
            let mut name = String::new();
            match request.as_reader().read_to_string(&mut name) {
                Ok(_) => {
                },
                Err(err) => {
                    println!("read request body error, err: {}", err);
                    break;
                }
            }
            dispatch.stopProcess(&name);
            break;
        }
    }

    pub fn handleRestartProcess(dispatch: &mut CDispatch, mut request: Request) {
        loop {
            let mut name = String::new();
            match request.as_reader().read_to_string(&mut name) {
                Ok(_) => {
                },
                Err(err) => {
                    println!("read request body error, err: {}", err);
                    break;
                }
            }
            dispatch.restartProcess(&name);
            break;
        }
    }

    pub fn handleStopAllProcess(dispatch: &mut CDispatch, request: Request) {
        loop {
            dispatch.stopAllProcess();
            break;
        }
    }

    pub fn handleRestartAllProcess(dispatch: &mut CDispatch, request: Request) {
        loop {
            dispatch.restartAllProcess();
            break;
        }
    }
}

impl CWebHandler {
    pub fn new() -> CWebHandler {
        CWebHandler{
        }
    }
}
