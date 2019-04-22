extern crate tiny_http;
extern crate chrono;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;
use std::time;
use chrono::prelude::*;
use chrono::{Duration, DateTime};

use tiny_http::{Server, Response, Method};

use std::process::{Command, Stdio};
use sysinfo::{ProcessExt, SystemExt, System, Signal};

use super::super::config::Process;
use super::super::templates::html;

pub struct CServer {
    // system: Arc<System>,
    system: System,
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
                    // Arc::get_mut(&mut self.system).unwrap().refresh_all();
                    self.system.refresh_all();
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
                            let mut procStatrTime = 0;
                            #[cfg(target_os="windows")]
                            {
                                procStatrTime = pro.start_time() as i64;
                            }
                            #[cfg(target_os="linux")]
                            {
                                let pid = pro.pid() as i32;
                                let mut path = String::new();
                                path.push_str("/proc/");
                                path.push_str(&pid.to_string());
                                println!("{:?}", path);
                                if let Ok(output) = Command::new("stat")
                                    .arg(path)
                                    .stdout(Stdio::piped())
                                    .output() {
                                    let result = String::from_utf8_lossy(&output.stdout);
                                    let lines: Vec<&str> = result.split("\n").collect();
                                    println!("{:?}", &lines);
                                    if lines.len() >= 5 {
                                        let access = lines[4].trim();
                                        let (key, value) = access.split_at("Access:".to_string().len());
                                        println!("{:?}", &value);
                                        let v = value.trim();
                                        let timePA: Vec<&str> = v.split(".").collect();
                                        println!("{:?}", &timePA);
                                        if timePA.len() >= 1 {
                                            let t = timePA[0];
                                            if let Ok(d) = chrono::DateTime::parse_from_str(t, "+%Y-%m-%d %H:%M:%S") {
                                                procStatrTime = d.timestamp();
                                                println!("{:?}", &procStatrTime);
                                            }
                                        }
                                    }
                                };
                                // let pid = pro.pid() as i32;
                                // let mut dir = String::new();
                                // dir.push_str("/proc/");
                                // dir.push_str(&pid.to_string());
                                // dir.push_str("/status");
                                // if let Ok(metadata) = fs::metadata(dir) {
                                //     if let Ok(t) = metadata.created() {
                                //         if let Ok(dur) = t.elapsed() {
                                //             procStatrTime = dur.as_secs() as i64;
                                //         }
                                //     }
                                // }
                            }
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
                    // Arc::get_mut(&mut self.system).unwrap().refresh_all();
                    self.system.refresh_all();
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
                    // Arc::get_mut(&mut self.system).unwrap().refresh_all();
                    self.system.refresh_all();
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

	pub fn new(processes: Arc<Vec<Process>>) -> CServer {
        let system = System::new();
		let server = CServer{
            system: system,
			processes: processes
		};
		server
	}
}
