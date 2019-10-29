use crate::core::dispatch::CDispatch;
use super::handlers::auth;
use super::handlers::index;
use super::handlers::web;
use super::handlers::api;

use tiny_http::{Server, Request, Response, Method, Header, StatusCode};

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::fs::File;
use std::thread;
use std::time;

use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(target_os="windows"))]
use signal_hook::flag as signal_flag;

pub struct CServer {
    dispatch: Arc<Mutex<CDispatch>>,
    // authHandler: auth::CAuthHandler,
    // indexHandler: index::CIndexHandler,
    // webHandler: web::CWebHandler,
    // apiHandler: api::CApiHandler
}

impl CServer {
	pub fn start(&mut self, inUser: &str, inPwd: &str, host: &str, port: u32, jsPath: &str) -> Result<(), &str> {
        let addr = self.joinAddr(host, port);
        {
            let mut dispatch = match self.dispatch.lock() {
                Ok(d) => {
                    d
                },
                Err(err) => {
                    return Err("lock error");
                }
            };
            dispatch.start();
        }
        let dispatch = self.dispatch.clone();
        let inUser = inUser.to_string();
        let inPwd = inPwd.to_string();
        let jsPath = jsPath.to_string();
        thread::spawn(move || {
            let server = match Server::http(addr) {
                Ok(s) => s,
                Err(err) => {
                    println!("http server start error, err: {}", err);
                    return;
                }
            };
            for mut request in server.incoming_requests() {
                let url = request.url();
                let method = request.method();
                let mut dispatch = match dispatch.lock() {
                    Ok(d) => {
                        d
                    },
                    Err(err) => {
                        return;
                    }
                };
                if *method == Method::Get && url == "/index" {
                    if !auth::CAuthHandler::handler(&inUser, &inPwd, request, |req: Request| {
                        index::CIndexHandler::handler(&dispatch, req);
                    }) {
                        continue;
                    }
                } else if *method == Method::Post && url == "/stop" {
                    web::CWebHandler::handleStopProcess(&mut dispatch, request);
                } else if *method == Method::Post && url == "/restart" {
                    web::CWebHandler::handleRestartProcess(&mut dispatch, request);
                } else if *method == Method::Post && url == "/stop/all" {
                    web::CWebHandler::handleStopAllProcess(&mut dispatch, request);
                } else if *method == Method::Post && url == "/restart/all" {
                    web::CWebHandler::handleRestartAllProcess(&mut dispatch, request);
                } else if *method == Method::Delete && url == "/api/stop" {
                    api::CApiHandler::handleStopProcess(&mut dispatch, request);
                } else if *method == Method::Delete && url == "/api/stop/with/config" {
                    api::CApiHandler::handleStopProcessWithConfig(&mut dispatch, request);
                } else if *method == Method::Put && url == "/api/restart" {
                    api::CApiHandler::handleRestartProcess(&mut dispatch, request);
                } else if *method == Method::Put && url == "/api/restart/with/config" {
                    api::CApiHandler::handleRestartProcessWithConfig(&mut dispatch, request);
                } else if *method == Method::Delete && url == "/api/stop/by/alias" {
                    api::CApiHandler::handleStopProcessByAlias(&mut dispatch, request);
                } else if *method == Method::Put && url == "/api/restart/by/alias" {
                    api::CApiHandler::handleRestartProcessByAlias(&mut dispatch, request);
                } else if *method == Method::Delete && url == "/api/stop/all" {
                    api::CApiHandler::handleStopAllProcess(&mut dispatch, request);
                } else if *method == Method::Put && url == "/api/restart/all" {
                    api::CApiHandler::handleRestartAllProcess(&mut dispatch, request);
                } else if *method == Method::Get && url == "/api/config" {
                    api::CApiHandler::handleGetAllConfig(&dispatch.fileOps(), request);
                } else if *method == Method::Put && url == "/api/reload" {
                    api::CApiHandler::handleReload(&mut dispatch, request);
                } else if *method == Method::Put && url == "/api/save/before/reload" {
                    api::CApiHandler::handleSaveBeforeReload(&mut dispatch, request);
                } else if *method == Method::Get && url == "/api/one/process/status" {
                    api::CApiHandler::handleGetOneStatusRequest(&dispatch, request);
                } else if *method == Method::Get && url == "/api/all/process/status" {
                    api::CApiHandler::handleGetAllStatusRequest(&dispatch, request);
                } else if *method == Method::Get && url == "/js/jquery-3.3.1.min.js" {
                    if let Ok(file) = File::open(&jsPath) {
                        request.respond(Response::from_file(file));
                    }
                } else if *method == Method::Get && url == "/favicon.ico" {
                    request.respond(Response::from_string("ok"));
                }
            }
        });
        #[cfg(not(target_os="windows"))]
        self.signalListen(self.dispatch.clone());
        Ok(())
	}

	pub fn new(mut dispatch: CDispatch) -> CServer {
		let server = CServer{
            dispatch: Arc::new(Mutex::new(dispatch)),
            // authHandler: auth::CAuthHandler::new(),
            // indexHandler: index::CIndexHandler::new(),
            // webHandler: web::CWebHandler::new(),
            // apiHandler: api::CApiHandler::new()
		};
		server
	}
}

impl CServer {
    #[cfg(not(target_os="windows"))]
    fn signalListen(&self, dispatch: Arc<Mutex<CDispatch>>) {
        let term = Arc::new(AtomicUsize::new(0));
        const SIGTERM: usize = signal_hook::SIGTERM as usize;
        const SIGINT: usize = signal_hook::SIGINT as usize;
        const SIGQUIT: usize = signal_hook::SIGQUIT as usize;
        signal_flag::register_usize(signal_hook::SIGTERM, Arc::clone(&term), SIGTERM).unwrap();
        signal_flag::register_usize(signal_hook::SIGINT, Arc::clone(&term), SIGINT).unwrap();
        signal_flag::register_usize(signal_hook::SIGQUIT, Arc::clone(&term), SIGQUIT).unwrap();

        loop {
            match term.load(Ordering::Relaxed) {
                0 => {
                    // Do some useful stuff here
                    time::Duration::from_millis(1000);
                },
                SIGTERM
                | SIGQUIT
                | SIGINT => {
                    eprintln!("Terminating on the TERM signal");
                    match dispatch.lock() {
                        Ok(mut d) => {
                            d.stopAllProcess();
                        },
                        Err(err) => {
                        }
                    };
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
}

impl CServer {
    fn joinAddr(&self, host: &str, port: u32) -> String {
        let mut addr = String::new();
        addr.push_str(host);
        addr.push_str(":");
        addr.push_str(&port.to_string());
        addr
    }
}
