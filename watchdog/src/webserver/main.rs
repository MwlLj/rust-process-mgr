extern crate tiny_http;
extern crate chrono;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time;
use chrono::prelude::*;
use chrono::{Duration, DateTime, NaiveDateTime};

use tiny_http::{Server, Response, Method, Header, StatusCode};

use std::process::{Command, Stdio};
use sysinfo::{ProcessExt, SystemExt, System, Signal};

use std::collections::VecDeque;

use base64;

use super::super::config::Process;
use super::super::templates::html;

const authorization: &'static str = "Authorization";

pub struct CServer {
    // system: Arc<System>,
    system: System,
	processes: Arc<Mutex<VecDeque<Process>>>
}

impl CServer {
	pub fn start(&mut self, inPwd: &str, host: &str, port: u32) {
		let mut addr = String::new();
		addr.push_str(host);
		addr.push_str(":");
		addr.push_str(&port.to_string());
		if let Ok(server) = Server::http(addr) {
			for mut request in server.incoming_requests() {
                let url = request.url();
                if *request.method() == Method::Get && (url.contains("/index")) {
                    // let (_, pwd) = url.split_at("/index?pwd=".to_string().len());
                    // Arc::get_mut(&mut self.system).unwrap().refresh_all();
                    // if inPwd != pwd {
                    let auth = CServer::findHeader(&request.headers(), authorization);
                    println!("{:?}", auth);
                    if *request.method() == Method::Get && auth == "" {
                        let h = Header::from_bytes("WWW-Authenticate", r#"Basic realm="Dotcoo User Login""#).unwrap();
                        let mut response = Response::from_data("");
                        let mut response = response.with_status_code(401);
                        response.add_header(h);
                        request.respond(response);
                        continue;
                    }
                    println!("{:?}", auth);
                    let v: Vec<&str> = auth.split(" ").collect();
                    if v.len() < 2 {
                        request.respond(Response::from_data("auth error"));
                        continue;
                    }
                    if v[0] != "Basic" {
                        request.respond(Response::from_data("auth is not Basic"));
                        continue;
                    }
                    let bytes = match base64::decode(v[1]) {
                        Ok(b) => b,
                        Err(_) => {
                            request.respond(Response::from_data("decode base64 error"));
                            continue;
                        }
                    };
                    let s = match String::from_utf8(bytes) {
                        Ok(s) => s,
                        Err(_) => {
                            request.respond(Response::from_data("from utf8 error"));
                            continue;
                        }
                    };
                    let v: Vec<&str> = s.split(":").collect();
                    if v.len() < 2 {
                        request.respond(Response::from_data("split by : error"));
                        continue;
                    }
                    if inPwd != v[1] {
                        request.respond(Response::from_string("password error"));
                        continue;
                    } else {
                        self.system.refresh_all();
                        let mut content = String::new();
                        content.push_str(html::htmlStartDefine);
                        if let Ok(p) = self.processes.lock() {
                            for item in &(*p) {
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
                                    if (cfg!(all(target_os="linux", target_arch="arm"))) {
                                        let pid = pro.pid() as i32;
                                        let mut path = String::new();
                                        path.push_str("/proc/");
                                        path.push_str(&pid.to_string());
                                        if let Ok(output) = Command::new("stat")
                                            .arg(path)
                                            .stdout(Stdio::piped())
                                            .output() {
                                            let result = String::from_utf8_lossy(&output.stdout);
                                            let lines: Vec<&str> = result.split("\n").collect();
                                            if lines.len() >= 5 {
                                                let access = lines[4].trim();
                                                let (key, value) = access.split_at("Access:".to_string().len());
                                                let v = value.trim();
                                                let timePA: Vec<&str> = v.split(".").collect();
                                                if timePA.len() >= 1 {
                                                    let t = timePA[0];
                                                    let parse_from_str = NaiveDateTime::parse_from_str;
                                                    if let Ok(d) = parse_from_str(t, "%Y-%m-%d %H:%M:%S") {
                                                        procStatrTime = d.timestamp();
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
                                    } else {
                                        procStatrTime = pro.start_time() as i64;
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
                        }
                        content.push_str(html::htmlEndDefine);

                        let response = Response::from_data(content);
                        request.respond(response);
                    }
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
                                if let Ok(mut p) = self.processes.lock() {
                                    let mut index = 0;
                                    for item in &(*p) {
                                        if item.name == processName {
                                            break;
                                        }
                                        index += 1;
                                    }
                                    if let Some(p) = p.get_mut(index) {
                                        (*p).isAuto = false;
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
                            if let Ok(mut p) = self.processes.lock() {
                                let mut index = 0;
                                for item in &(*p) {
                                    if item.name == processName {
                                        break;
                                    }
                                    index += 1;
                                }
                                if let Some(p) = p.get_mut(index) {
                                    (*p).isAuto = true;
                                }
                            }
                            request.respond(Response::from_string("success"));
                        } else {
                            let pro = process[0];
                            if pro.kill(Signal::Kill) {
                                if let Ok(mut p) = self.processes.lock() {
                                    let mut index = 0;
                                    for item in &(*p) {
                                        if item.name == processName {
                                            break;
                                        }
                                        index += 1;
                                    }
                                    if let Some(p) = p.get_mut(index) {
                                        (*p).isAuto = true;
                                    }
                                }
                                request.respond(Response::from_string("success"));
                            }
                        }
                    }
                } else if *request.method() == Method::Post && request.url() == "/stop/all" {
                    println!("stop all");
                    self.system.refresh_all();
                    if let Ok(mut p) = self.processes.lock() {
                        for mut item in &mut (*p) {
                            let process = self.system.get_process_by_name(&item.name);
                            if process.len() == 0 {
                            } else {
                                let pro = process[0];
                                if pro.kill(Signal::Kill) {
                                    (*item).isAuto = false;
                                }
                            }
                        }
                        request.respond(Response::from_string("success"));
                    } else {
                        request.respond(Response::from_string("error"));
                    }
                } else if *request.method() == Method::Post && request.url() == "/restart/all" {
                    println!("restart all");
                    self.system.refresh_all();
                    if let Ok(mut p) = self.processes.lock() {
                        for mut item in &mut (*p) {
                            let process = self.system.get_process_by_name(&item.name);
                            if process.len() == 0 {
                                (*item).isAuto = true;
                            } else {
                                let pro = process[0];
                                if pro.kill(Signal::Kill) {
                                    (*item).isAuto = true;
                                }
                            }
                        }
                        request.respond(Response::from_string("success"));
                    } else {
                        request.respond(Response::from_string("error"));
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

	pub fn new(processes: Arc<Mutex<VecDeque<Process>>>) -> CServer {
        let system = System::new();
		let server = CServer{
            system: system,
			processes: processes
		};
		server
	}
}

impl CServer {
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
