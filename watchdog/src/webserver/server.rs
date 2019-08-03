use crate::core::dispatch::CDispatch;
use super::handlers::auth;
use super::handlers::index;
use super::handlers::web;
use super::handlers::api;

use tiny_http::{Server, Request, Response, Method, Header, StatusCode};

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::fs::File;

pub struct CServer {
    dispatch: CDispatch,
    authHandler: auth::CAuthHandler,
    indexHandler: index::CIndexHandler,
    webHandler: web::CWebHandler,
    apiHandler: api::CApiHandler
}

impl CServer {
	pub fn start(&mut self, inPwd: &str, host: &str, port: u32, jsPath: &str) -> Result<(), &str> {
        let addr = self.joinAddr(host, port);
        let server = match Server::http(addr) {
            Ok(s) => s,
            Err(err) => {
                println!("http server start error, err: {}", err);
                return Err("http server start error");
            }
        };
        for mut request in server.incoming_requests() {
            let url = request.url();
            let method = request.method();
            if *method == Method::Get && url == "/index" {
                if !self.authHandler.handler(inPwd, request, |req: Request| {
                    self.indexHandler.handler(&self.dispatch, req);
                }) {
                    continue;
                }
            } else if *method == Method::Post && url == "/stop" {
                self.webHandler.handleStopProcess(&mut self.dispatch, request);
            } else if *method == Method::Post && url == "/restart" {
                self.webHandler.handleRestartProcess(&mut self.dispatch, request);
            } else if *method == Method::Post && url == "/stop/all" {
                self.webHandler.handleStopAllProcess(&mut self.dispatch, request);
            } else if *method == Method::Post && url == "/restart/all" {
                self.webHandler.handleRestartAllProcess(&mut self.dispatch, request);
            } else if *method == Method::Delete && url == "/api/stop" {
                self.apiHandler.handleStopProcess(&mut self.dispatch, request);
            } else if *method == Method::Post && url == "/api/restart" {
                self.apiHandler.handleRestartProcess(&mut self.dispatch, request);
            } else if *method == Method::Delete && url == "/api/stop/all" {
                self.apiHandler.handleStopAllProcess(&mut self.dispatch, request);
            } else if *method == Method::Post && url == "/api/restart/all" {
                self.apiHandler.handleRestartAllProcess(&mut self.dispatch, request);
            } else if *method == Method::Get && url == "/api/config" {
                self.apiHandler.handleGetAllConfig(&self.dispatch.fileOps(), request);
            } else if *method == Method::Get && url == "/js/jquery-3.3.1.min.js" {
                if let Ok(file) = File::open(jsPath) {
                    request.respond(Response::from_file(file));
                }
            } else if *method == Method::Get && url == "/favicon.ico" {
                request.respond(Response::from_string("ok"));
            }
        }
        Ok(())
	}

	pub fn new(mut dispatch: CDispatch) -> CServer {
        dispatch.start();
		let server = CServer{
            dispatch: dispatch,
            authHandler: auth::CAuthHandler::new(),
            indexHandler: index::CIndexHandler::new(),
            webHandler: web::CWebHandler::new(),
            apiHandler: api::CApiHandler::new()
		};
		server
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
