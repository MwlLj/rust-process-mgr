extern crate tiny_http;

use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use tiny_http::{Server, Response, Method};

use std::process::Command;
use sysinfo::{ProcessExt, SystemExt, System};

use super::super::config::Process;
use super::super::templates::html;

pub struct CServer {
    system: System,
	processes: Arc<Vec<Process>>
}

impl CServer {
	pub fn start(&self, host: &str, port: u32) {
		let mut addr = String::new();
		addr.push_str(host);
		addr.push_str(":");
		addr.push_str(&port.to_string());
		if let Ok(server) = Server::http(addr) {
			for request in server.incoming_requests() {
                if *request.method() == Method::Get && request.url() == "/" {
                    let mut content = String::new();
                    content.push_str(html::htmlStartDefine);
                    for item in &(*self.processes) {
                        // state display
                        let process = self.system.get_process_by_name(&item.name);
                        if process.len() == 0 {
                            content.push_str("obj.state = 'stopped';");
                            content.push_str("obj.description = 'unknow';");
                        } else {
                            content.push_str("obj.state = 'running';");
                            // desc display
                            let pro = process[0];
                            content.push_str("obj.description = '");
                            content.push_str("pid: ");
                            content.push_str(&pro.pid().to_string());
                            content.push_str("';");
                        }
                        // name display
                        content.push_str(&format!("obj.name = '{}';", item.name));
                        // display
                        content.push_str("create(obj);");
                    }
                    content.push_str(html::htmlEndDefine);

                    let response = Response::from_data(content);
                    request.respond(response);
                }
			}
		} else {
			println!("addr error");
		}
	}

	pub fn new(processes: Arc<Vec<Process>>) -> CServer {
        let system = sysinfo::System::new();
		let server = CServer{
            system: system,
			processes: processes
		};
		server
	}
}
