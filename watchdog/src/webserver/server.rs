extern crate tiny_http;
extern crate chrono;

use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use chrono::prelude::*;
use chrono::Duration;

use tiny_http::{Server, Response, Method};

use std::process::Command;
use sysinfo::{ProcessExt, SystemExt, System, Signal};

use super::super::config::Process;
use super::super::templates::html;

pub struct CServer {
    system: Arc<System>,
	processes: Arc<Vec<Process>>
}

impl CServer {
	pub fn start(&mut self, host: &str, port: u32) {
		let mut addr = String::new();
		addr.push_str(host);
		addr.push_str(":");
		addr.push_str(&port.to_string());
		if let Ok(server) = Server::http(addr) {
			for mut request in server.incoming_requests() {
                if *request.method() == Method::Get && request.url() == "/" {
                    Arc::get_mut(&mut self.system).unwrap().refresh_all();
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
                            // pid
                            content.push_str("pid: ");
                            content.push_str(&pro.pid().to_string());
                            // run time
                            let procStatrTime = pro.start_time() as i64;
                            let dt = Local::now();
                            let now = dt.timestamp();
                            let sub = now - procStatrTime;
                            content.push_str(", runtime: ");
                            content.push_str(&self.calcSec2DHMS(sub));
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
                } else if *request.method() == Method::Post && request.url() == "/stop" {
                    Arc::get_mut(&mut self.system).unwrap().refresh_all();
                    let mut processName = String::new();
                    if let Ok(_) = request.as_reader().read_to_string(&mut processName) {
                        let process = self.system.get_process_by_name(&processName);
                        if process.len() == 0 {
                        } else {
                            let pro = process[0];
                            if pro.kill(Signal::Kill) {
                                if let Some(p) = Arc::get_mut(&mut self.processes) {
                                    println!("some is not null");
                                    for mut item in p {
                                        if item.name == processName {
                                            println!("set isAuto false");
                                            (*item).isAuto = false;
                                            break;
                                        }
                                    }
                                }
                                request.respond(Response::from_string("success"));
                            }
                        }
                    }
                } else if *request.method() == Method::Post && request.url() == "/restart" {
                    Arc::get_mut(&mut self.system).unwrap().refresh_all();
                    let mut processName = String::new();
                    if let Ok(_) = request.as_reader().read_to_string(&mut processName) {
                        let process = self.system.get_process_by_name(&processName);
                        if process.len() == 0 {
                        } else {
                            let pro = process[0];
                            if pro.kill(Signal::Kill) {
                                if let Some(p) = Arc::get_mut(&mut self.processes) {
                                    println!("some is not null");
                                    for mut item in p {
                                        if item.name == processName {
                                            println!("set isAuto false");
                                            (*item).isAuto = false;
                                            break;
                                        }
                                    }
                                }
                                request.respond(Response::from_string("success"));
                            }
                        }
                    }
                } else if *request.method() == Method::Get && request.url() == "/js/jquery-3.3.1.min.js" {
                    if let Ok(file) = File::open("js/jquery-3.3.1.min.js") {
                        request.respond(Response::from_file(file));
                    }
                } else if *request.method() == Method::Get && request.url() == "/favicon.ico" {
                    request.respond(Response::from_string("ok"));
                }
			}
		} else {
			println!("addr error");
		}
	}

    fn calcSec2DHMS(&self, sec: i64) -> String {
        let mut result = String::new();
        let dur = Duration::seconds(sec);
        result.push_str(&dur.num_days().to_string());
        result.push_str("day, ");
        result.push_str(&(dur.num_hours()%24).to_string());
        result.push_str(":");
        result.push_str(&(dur.num_minutes()%60).to_string());
        result.push_str(":");
        result.push_str(&(dur.num_seconds()%60).to_string());
        // result.push_str(&sec.to_string());
        // result.push_str("s");
        result
    }

	pub fn new(system: Arc<System>, processes: Arc<Vec<Process>>) -> CServer {
        // let system = sysinfo::System::new();
		let server = CServer{
            system: system,
			processes: processes
		};
		server
	}
}
