use crate::templates::html;
use crate::process::ProcessStatus;
use crate::core::dispatch::CDispatch;

use tiny_http::{Request, Response};

pub struct CWebHandler {
}

impl CWebHandler {
    pub fn handleStopProcess(&self, dispatch: &mut CDispatch, mut request: Request) {
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

    pub fn handleRestartProcess(&self, dispatch: &mut CDispatch, mut request: Request) {
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

    pub fn handleStopAllProcess(&self, dispatch: &mut CDispatch, request: Request) {
        loop {
            dispatch.stopAllProcess();
            break;
        }
    }

    pub fn handleRestartAllProcess(&self, dispatch: &mut CDispatch, request: Request) {
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
